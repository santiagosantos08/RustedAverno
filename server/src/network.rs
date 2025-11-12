use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use crate::db::*;
use crate::hasher::*;
use anyhow::Result;
use bincode;
use common::MessageTypeClientToServer::Auth;
use common::{ConnectionState, MessageTypeClientToServer, MessageTypeServerToClient};
use gns::sys::*;
use gns::*;
use log::*;
use serde::{Deserialize, Serialize};
use sqlx::Statement;
use tokio::runtime::Runtime;
use tokio_postgres::types::ToSql;

struct ConnectedClient {
    is_authed: bool,
    db_id: u32,
    last_known_world_tick: u64,
}
impl ConnectedClient {
    pub fn new() -> Self {
        Self {
            is_authed: false,
            db_id: 0,
            last_known_world_tick: 0,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AuthRequest {
    tmp_id: u64,
    username: String,
    provided_password: String,
    db_id: u32,
    db_hash: String,
}

impl AuthRequest {
    pub fn new(
        tmp_id: u64,
        username: String,
        provided_password: String,
        db_id: u32,
        db_hash: String,
    ) -> Self {
        Self {
            tmp_id: tmp_id,
            username: username,
            provided_password: provided_password,
            db_id: db_id,
            db_hash: db_hash,
        }
    }
}

pub struct ServerNetwork {
    // state
    should_shutdown: Arc<Mutex<bool>>,
    inbound: Arc<Mutex<VecDeque<MessageTypeClientToServer>>>,
    outbound: Arc<Mutex<VecDeque<MessageTypeClientToServer>>>,
}

impl ServerNetwork {
    pub fn new() -> Self {
        Self {
            should_shutdown: Arc::new(Mutex::new(false)),
            inbound: Arc::new(Mutex::new(VecDeque::new())),
            outbound: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    pub fn start(&self) -> thread::JoinHandle<()> {
        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // THREAD SETUP
        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        let should_shutdown = Arc::clone(&self.should_shutdown);
        let inbound = Arc::clone(&self.inbound);
        let outbound = Arc::clone(&self.outbound);
        let mut temp_id_index: u64 = 0;

        let mut prepared: HashMap<DbStmt,&str> = HashMap::new();
        prepared.insert(DbStmt::GetUser,r#"SELECT * FROM "USER" WHERE userName = $1"#);

        let db_worker = DbWorker::new(
            "postgres://avernogameserver:NAOOO@192.168.0.150:5432/avernodb",
            prepared,
        );

        let mut argon_worker = Argon2Worker::new(1);

        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // START OS THREAD
        //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        thread::spawn(move || {


            let mut new_clients: HashMap<GnsConnection, AuthRequest> = HashMap::new();
            let mut db_issued_clients: HashMap<GnsConnection, AuthRequest> = HashMap::new();
            let mut auth_issued_clients: HashMap<GnsConnection, AuthRequest> = HashMap::new();
            let mut authed_clients: HashMap<GnsConnection, ConnectedClient> = HashMap::new();
            let mut ingame_clients: HashMap<GnsConnection, ConnectedClient> = HashMap::new();
            info!("client: network thread starting -> {}", Ipv4Addr::LOCALHOST);
            let mut quit = false;
            let gns_global = GnsGlobal::get().unwrap();

            // Setup debugging to log everything.
            // The current rust implementation flush the log in stdout.
            gns_global.utils().enable_debug_output(
                ESteamNetworkingSocketsDebugOutputType::k_ESteamNetworkingSocketsDebugOutputType_Everything,
                |ty, message| println!("{:#?}: {}", ty, message),
            );

            // Add fake 1000ms ping to everyone connecting.
            // **unwrap** must be banned in production.
            gns_global
                .utils()
                .set_global_config_value(
                    ESteamNetworkingConfigValue::k_ESteamNetworkingConfig_FakePacketLag_Recv,
                    GnsConfig::Int32(5),
                )
                .unwrap();

            let server = GnsSocket::new(gns_global.clone())
                .listen(Ipv4Addr::LOCALHOST.into(), 3750)
                // **unwrap** must be banned in production.
                .unwrap();

            'net_loop: loop {
                std::thread::sleep(Duration::from_millis(10));
                gns_global.poll_callbacks();
                //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                // EVENT POLLING
                //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                let mut events: Vec<GnsConnectionEvent> = Vec::new();
                server.poll_event::<100>(|event| {
                    events.push(event);
                });
                for event in events {
                    match (event.old_state(), event.info().state()) {
                        // A client is about to connect, accept it.
                        (
                            ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_None,
                            ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connecting,
                        ) => {
                            let result = server.accept(event.connection());
                            if result.is_ok() {
                                println!("GnsSocket<Server>: accepted new REALLY client: {:#?}.", result);
                                new_clients.insert(event.connection(), AuthRequest::new(temp_id_index,"".to_string(),"".to_string(),0,"".to_string()));
                                temp_id_index = temp_id_index +1;
                            }
                        }

                        // A client is connected, we previously accepted it and don't do anything here.
                        // In a more sophisticated scenario we could initial sending some messages.
                        (
                            ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connecting,
                            ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connected,
                        ) => {
                        }

                        (_, ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_ClosedByPeer | ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_ProblemDetectedLocally) => {
                            // Remove the client from the list and close the connection.
                            let conn = event.connection();
                            println!("GnsSocket<Server>: {:#?} disconnected", conn);
                            // TODO buscar en todos los hashmap y dropear del q haga falta
                            new_clients.remove(&conn);
                            authed_clients.remove(&conn);
                            auth_issued_clients.remove(&conn);
                            ingame_clients.remove(&conn);
                            // Make sure we cleanup the connection, mandatory as per GNS doc.
                            server.close_connection(conn, 0, "", false);
                        }

                        // A client state is changing, perhaps disconnecting
                        // If a client disconnected and it's connection get cleaned up, its state goes back to `ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_None`
                        (previous, current) => {
                            println!("GnsSocket<Server>: {:#?} => {:#?}.", previous, current);
                        }
                    }
                }

                //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                // CHECK AUTH STATES AND REPLY ACCORDINGLY
                //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                //poll db results.
                let db_results: Vec<_> = {
                    let mut results = Vec::new();
                    while let Some(result) = db_worker.poll_result_sync() {
                        results.push(result);
                    }
                    results
                };
                // Process without holding any locks
                for result in db_results {
                    match result.stmt {
                        DbStmt::GetUser => {
                            argon_worker.queue_job(
                                r#"*6jtfaTAMW".8m2"#,
                                "$argon2id$v=19$m=65536,t=3,p=4$sxJW1fzX7YL6/JY/ldpdzA$3jgVv6R8Gz+Ubz5IOKYPFmNXKhlxxQPwFa01ifYoo+0",
                                result.id,
                                result.respond_to
                            );
                            if let Some(auth_req) = db_issued_clients.get(&result.respond_to) {
                                auth_issued_clients.insert(result.respond_to, auth_req.clone());
                                db_issued_clients.remove(&result.respond_to);
                                println!("Issued to Argon");
                            }
                        }
                        _ => warn!("UNKNOWN STATEMENT")
                    }
                }
                // Drain argon results without holding lock
                let argon_results: Vec<_> = {
                    let mut results = Vec::new();
                    while let Some(result) = argon_worker.poll_result_sync() {
                        results.push(result);
                    }
                    results
                };

                // Process results without holding any locks
                for result in argon_results {
                    if result.ok {
                        match bincode::serialize(&MessageTypeServerToClient::AuthOk) {
                            Ok(res) => {
                                server.send_messages(vec![gns_global.utils().allocate_message(
                                    result.respond_to,
                                    k_nSteamNetworkingSend_Reliable,
                                    &res,
                                )]);

                                // Move client to authed_clients if needed
                                if let Some(auth_req) = auth_issued_clients.remove(&result.respond_to) {
                                    authed_clients.insert(result.respond_to, ConnectedClient::new());
                                    println!("Client authenticated successfully");
                                }

                                println!("Sent auth OK response");
                            }
                            Err(e) => {
                                error!("Failed to serialize AuthOk message: {:?}", e);
                                // Consider sending an error response to client instead of panicking
                            }
                        }
                    } else {
                        // Authentication failed
                        warn!("Authentication failed for client {:?}", result.respond_to);

                        // Send auth failure response
                        if let Ok(res) = bincode::serialize(&MessageTypeServerToClient::AuthOk) {
                            server.send_messages(vec![gns_global.utils().allocate_message(
                                result.respond_to,
                                k_nSteamNetworkingSend_Reliable,
                                &res,
                            )]);
                        }

                        // Remove from auth_issued_clients, keep in new_clients or disconnect
                        auth_issued_clients.remove(&result.respond_to);
                    }
                }




                //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                // INCOMING CLIENT MESSAGES POLLING
                //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
                // Process some messages, we arbitrary define 100 as being the max number of messages we can handle per iteration.
                let _messages_processed = server.poll_messages::<100>(|message| {
                    // **unwrap** must be banned in production.
                    if let Ok(chat_message) = bincode::deserialize::<MessageTypeClientToServer>(message.payload()){
                        match chat_message {
                            MessageTypeClientToServer::Auth { token } => {
                                println!("DETENTO BIEN QUE EL TIPO DE MENSAHE ES AUTH");
                                if let Some(auth_req) = new_clients.get(&message.connection()){
                                    let issued = db_worker.queue_job(auth_req.tmp_id, DbStmt::GetUser, vec!["Odrish".to_string()],message.connection());
                                    if issued{
                                        db_issued_clients.insert(message.connection(), auth_req.clone());
                                        new_clients.remove(&message.connection());
                                        println!("Issued to DB");
                                    } else {
                                        // TODO requeue....
                                    }
                                };
                            }
                            MessageTypeClientToServer::Ping { .. } => {}
                            MessageTypeClientToServer::PlayerMove { x, y } => {}
                        }
                    };

                    let sender = message.connection();
                    //let sender_nickname = &connected_clients[&sender];
                });


            }
        })
    }

    pub fn shutdown(&self) {
        *self.should_shutdown.lock().unwrap() = true;
    }
}

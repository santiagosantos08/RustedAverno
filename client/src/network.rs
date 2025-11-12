use std::any::Any;
use std::collections::VecDeque;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use bincode;
use common::{ConnectionState, MessageTypeClientToServer, MessageTypeServerToClient};
use gns::sys::*;
use gns::*;
use log::*;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

pub struct ClientNetwork {
    // state
    should_shutdown: Arc<Mutex<bool>>,
    inbound: Arc<Mutex<VecDeque<MessageTypeServerToClient>>>,
    outbound: Arc<Mutex<VecDeque<MessageTypeClientToServer>>>,
    connection_state: Arc<Mutex<ConnectionState>>,
}

impl ClientNetwork {
    pub fn new() -> Self {
        Self {
            should_shutdown: Arc::new(Mutex::new(false)),
            inbound: Arc::new(Mutex::new(VecDeque::new())),
            outbound: Arc::new(Mutex::new(VecDeque::new())),
            connection_state: Arc::new(Mutex::new(ConnectionState::NetworkUninitialized)),
        }
    }

    pub fn start(&self, server_addr: IpAddr) -> thread::JoinHandle<()> {
        let should_shutdown = Arc::clone(&self.should_shutdown);
        let inbound = Arc::clone(&self.inbound);
        let outbound = Arc::clone(&self.outbound);
        let connection_state = Arc::clone(&self.connection_state);

        thread::spawn(move || {
            info!("client: network thread starting -> {}", server_addr);
            let mut quit = false;

            // Initialize gns global and enable debugging
            let gns_global = gns::GnsGlobal::get().expect("gns init");
            gns_global.utils().enable_debug_output(
                ESteamNetworkingSocketsDebugOutputType::k_ESteamNetworkingSocketsDebugOutputType_Everything,
                |ty, message| println!("{:#?}: {}", ty, message),
            );
            // Create client socket
            let client_socket = gns::GnsSocket::new(gns_global.clone()).connect(server_addr, 3750).unwrap();

            'net_loop: loop {
                gns_global.poll_callbacks();

                // Poll incoming events and match status change.
                let _ = client_socket.poll_event::<100>(|event| match (event.old_state(), event.info().state()) {
                    (
                        ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_None,
                        ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connecting,
                    ) => {
                        println!("GnsSocket<Client>: connecting to server.");
                        *connection_state.lock().unwrap() = ConnectionState::Connecting;
                    }
                    (
                        ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connecting,
                        ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_Connected,
                    ) => {
                        println!("GnsSocket<Client>: connected to server.");
                        *connection_state.lock().unwrap() = ConnectionState::Connected

                    }
                    (_, ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_ClosedByPeer | ESteamNetworkingConnectionState::k_ESteamNetworkingConnectionState_ProblemDetectedLocally) => {
                        // We got disconnected or lost the connection.
                        println!("GnsSocket<Client>: ET phone home.");
                        quit = true;
                    }
                    (previous, current) => {
                        println!("GnsSocket<Client>: {:#?} => {:#?}.", previous, current);
                    }
                });

                // Poll incoming messages and queue to inbound.
                let _ = client_socket.poll_messages::<100>(|message| {
                    if let Ok(smsg) = bincode::deserialize::<MessageTypeServerToClient>(message.payload())
                    {
                        match smsg {
                            MessageTypeServerToClient::AuthOk => {
                                println!("GnsSocket<Client>: auth ok form server!");
                            }
                            MessageTypeServerToClient::GameState {..} => {

                            }
                            MessageTypeServerToClient::Pong {..} => {

                            }
                        }
                        inbound.lock().unwrap().push_back(smsg);
                    }
                });

                let mut q = outbound.lock().unwrap();

                if *connection_state.lock().unwrap() == ConnectionState::Connected{
                    while let Some(msg) = q.pop_front() {
                        let msg_as_bytes_res = bincode::serialize(&msg);
                        match msg_as_bytes_res {
                            Ok(message) => {
                                println!("sent message to server");
                                client_socket.send_messages(vec![gns_global.utils().allocate_message(
                                    client_socket.connection(),
                                    k_nSteamNetworkingSend_Reliable,
                                    &message,
                                )]);
                            }
                            Err(e) => {
                                warn!("client: failed to serialize msg: {:?}", e);
                            }
                        };
                    }
                }

                if *should_shutdown.lock().unwrap() == true {
                    client_socket.close_connection(client_socket.connection(),0,"asd",true);
                    print!("closing connection");
                    break 'net_loop;
                }
                std::thread::sleep(Duration::from_millis(10))
            }
        }
    )
}

pub fn queue_send(&self, msg: MessageTypeClientToServer) {
    self.outbound.lock().unwrap().push_back(msg);
}

pub fn poll_inbound(&self) -> Vec<MessageTypeServerToClient> {
    let mut out = Vec::new();
    let mut q = self.inbound.lock().unwrap();
    while let Some(m) = q.pop_front() {
        out.push(m);
    }
    out
}

pub fn shutdown(&self) {
    *self.should_shutdown.lock().unwrap() = true;
    println!("client: network thread shutting down");
}
}

mod network;
mod db;
mod hasher;
use argon2::password_hash::PasswordVerifier;
use std::time::Duration;
use std::time::Instant;

use tokio::runtime::Builder;
use tokio::task;
use tokio::time;
use crate::network::ServerNetwork;
use core::net::IpAddr;

fn main() {

    let net : ServerNetwork = ServerNetwork::new();
    let net_join_handle = net.start();
    net_join_handle.join();

    /*
    println!("--- Building Tokio Runtime Explicitly ---");
    let runtime = Builder::new_multi_thread()
        .enable_all() // Enable IO, time, etc.
        .build()
        .unwrap();

    println!("--- Starting Runtime with .block_on() ---");
    runtime.block_on(async {
        let start_time = Instant::now();
        let stored_hash = "$argon2id$v=19$m=65536,t=3,p=4$vtTMrpk2S0h9OvMHiz+DtA$Qg03oNiqec6Mk2RRsd67xMjOtC6Q7XaOATVcB0z8dx8".to_string();
        let mut game_loop_interval = time::interval(Duration::from_millis(500));
        let mut tick_count = 0;

        println!("[Main Task] Starting async 'game loop'...\n");

        loop {
            game_loop_interval.tick().await;
            let elapsed = start_time.elapsed().as_secs_f32();
            println!("[Game Loop] Tick: {}, Time: {:.1}s", tick_count, elapsed);
            if tick_count == 3 {
                println!("\n[Game Loop] == Spawning 'correct password' task ==\n");
                let hash_clone = stored_hash.clone();
                let password_clone = "password123".to_string();
                task::spawn(async move {
                    println!("[Verifier Task 1] SPAWNED. Starting slow verification...");
                    match verify_password(password_clone, hash_clone).await {
                        Ok(true) => println!("[Verifier Task 1] FINISHED. Result: Success!"),
                        Ok(false) => println!("[Verifier Task 1] FINISHED. Result: Failure."),
                        Err(e) => println!("[Verifier Task 1] FINISHED. Error: {}", e),
                    }
                });
            }
            if tick_count == 5 {
                println!("\n[Game Loop] == Spawning 'wrong password' task ==\n");
                let hash_clone = stored_hash.clone();
                let password_clone = "wrongpassword".to_string();

                task::spawn(async move {
                    println!("[Verifier Task 2] SPAWNED. Starting slow verification...");
                    match verify_password(password_clone, hash_clone).await {
                        Ok(true) => println!("[Verifier Task 2] FINISHED. Result: Success!"),
                        Ok(false) => println!("[Verifier Task 2] FINISHED. Result: Failure."),
                        Err(e) => println!("[Verifier Task 2] FINISHED. Error: {}", e),
                    }
                });
            }
            if tick_count > 20 {
                println!("[Game Loop] Demo finished. Exiting.");
                break;
            }

            tick_count += 1;
        }
    });

     */

    println!("--- Runtime has shut down. Program exiting. ---");
}
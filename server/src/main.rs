mod network;
mod db;
mod hasher;
use crate::hasher::verify_password;
use argon2::password_hash::PasswordVerifier;
use std::time::Duration;
use std::time::Instant;

use tokio::runtime::Builder;
use tokio::task;
use tokio::time;

fn main() {
    println!("--- Building Tokio Runtime Explicitly ---");
    // 1. This is the "magic" that #[tokio::main] hides.
    // We are manually building the multi-threaded runtime.
    // This runtime IS the event loop that "does other things".
    let runtime = Builder::new_multi_thread()
        .enable_all() // Enable IO, time, etc.
        .build()
        .unwrap();

    println!("--- Starting Runtime with .block_on() ---");
    // 2. We tell the main thread to block and hand control to the
    // runtime, giving it our main async task to run.
    runtime.block_on(async {
        // This is our main async task, which now runs on the runtime

        // --- Setup for our demo ---
        let start_time = Instant::now();
        let stored_hash = "$argon2id$v=19$m=65536,t=3,p=4$vtTMrpk2S0h9OvMHiz+DtA$Qg03oNiqec6Mk2RRsd67xMjOtC6Q7XaOATVcB0z8dx8".to_string();

        // This is our "game loop" timer. It ticks every 500ms.
        let mut game_loop_interval = time::interval(Duration::from_millis(500));
        let mut tick_count = 0;

        println!("[Main Task] Starting async 'game loop'...\n");

        // --- The Actual Game Loop ---
        loop {
            // .tick().await is the non-blocking pause.
            // When this task hits .await, it yields control back to the
            // runtime, which can then work on other tasks (like our verifier).
            game_loop_interval.tick().await;

            let elapsed = start_time.elapsed().as_secs_f32();
            println!("[Game Loop] Tick: {}, Time: {:.1}s", tick_count, elapsed);

            // --- Trigger the password checks without blocking ---

            // At tick 3 (around 1.5s), spawn the "correct password" check
            if tick_count == 3 {
                println!("\n[Game Loop] == Spawning 'correct password' task ==\n");
                let hash_clone = stored_hash.clone();
                let password_clone = "password123".to_string();

                // tokio::task::spawn() creates a NEW, independent task
                // The game loop does NOT await this. It just starts it
                // and immediately continues to its next loop.
                task::spawn(async move {
                    println!("[Verifier Task 1] SPAWNED. Starting slow verification...");
                    match verify_password(password_clone, hash_clone).await {
                        Ok(true) => println!("[Verifier Task 1] FINISHED. Result: Success!"),
                        Ok(false) => println!("[Verifier Task 1] FINISHED. Result: Failure."),
                        Err(e) => println!("[Verifier Task 1] FINISHED. Error: {}", e),
                    }
                });
            }

            // At tick 5 (around 2.5s), spawn the "wrong password" check
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

            // Stop the demo after 10 seconds
            if tick_count > 20 {
                println!("[Game Loop] Demo finished. Exiting.");
                break;
            }

            tick_count += 1;
        }
    });

    println!("--- Runtime has shut down. Program exiting. ---");
}
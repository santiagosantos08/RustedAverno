// All the 'use' statements needed by verify_password are now in this file.
use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordVerifier;
use argon2::Argon2;
use std::io::{Error, ErrorKind};
use tokio::sync::oneshot;
use tokio::task;

/// Verifies a password against an Argon2 hash in a non-blocking way
/// using an explicit oneshot channel.
pub async fn verify_password(
    password_str: String,
    hash_str: String,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {

    // 1. Create the explicit one-time-use channel
    let (tx, rx) = oneshot::channel::<Result<bool, std::io::Error>>();

    // 2. Spawn the blocking task, moving the sender (tx) into it
    task::spawn_blocking(move || {
        // 1. Parse the hash string.
        let parsed_hash = match PasswordHash::new(&hash_str) {
            Ok(hash) => hash,
            Err(e) => {
                let _ = tx.send(Err(Error::new(ErrorKind::InvalidInput, e.to_string())));
                return;
            }
        };

        // 2. Perform the verification.
        let argon2 = Argon2::default();
        let result = match argon2.verify_password(password_str.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true), // Password matches
            Err(argon2::password_hash::Error::Password) => Ok(false), // Password mismatch
            Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
        };

        // 3. Send the final result back to the async task.
        let _ = tx.send(result);
    }); // The JoinHandle is dropped here, we don't await it

    match rx.await {
        Ok(verification_result) => {
            verification_result.map_err(|e| {
                let boxed_error: Box<dyn std::error::Error + Send + Sync> = Box::new(e);
                boxed_error
            })
        },
        Err(e) => {
            let boxed_error: Box<dyn std::error::Error + Send + Sync> =
                Box::new(Error::new(ErrorKind::BrokenPipe, e.to_string()));
            Err(boxed_error)
        }
    }
}
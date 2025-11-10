use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordVerifier;
use argon2::Argon2;
use std::io::{Error, ErrorKind};
use tokio::sync::oneshot;
use tokio::task;

// Verifies a password against an Argon2 hash in a non-blocking way
pub async fn verify_password(
    password_str: String,
    hash_str: String,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let (tx, rx) = oneshot::channel::<Result<bool, std::io::Error>>();

    task::spawn_blocking(move || {
        let parsed_hash = match PasswordHash::new(&hash_str) {
            Ok(hash) => hash,
            Err(e) => {
                let _ = tx.send(Err(Error::new(ErrorKind::InvalidInput, e.to_string())));
                return;
            }
        };

        let argon2 = Argon2::default();
        let result = match argon2.verify_password(password_str.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true), // Password matches
            Err(argon2::password_hash::Error::Password) => Ok(false), // Password mismatch
            Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
        };

        let _ = tx.send(result);
    }); // The JoinHandle is dropped here

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
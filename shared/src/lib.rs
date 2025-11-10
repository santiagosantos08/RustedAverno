use serde::{Deserialize, Serialize};

mod network;
mod game;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageTypeClientToServer {
    Auth { token: String },
    PlayerMove { x: f32, y: f32 },
    Ping(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageTypeServerToClient {
    AuthOk,
    Pong(u64),
    GameState { players: Vec<(u64, f32, f32)> },
}
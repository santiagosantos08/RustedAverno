use std::collections::VecDeque;
use std::thread;
use std::sync::{Arc,Mutex};
use steamworks::{Client};

pub struct ClientNetwork{
    pub should_shut_down_thread: Arc<Mutex<bool>>,
    pub is_connected: Arc<Mutex<bool>>,
    pub is_authenticated: Arc<Mutex<bool>>,
    inbound_queue: Arc<Mutex<VecDeque<i32>>>,
    outbound_queue: Arc<Mutex<VecDeque<i32>>>,
}

impl ClientNetwork{
    pub fn new() -> Self {
        ClientNetwork{
            should_shut_down_thread: Arc::new(Mutex::new(false)),
            is_connected: Arc::new(Mutex::new(false)),
            is_authenticated: Arc::new(Mutex::new(false)),
            inbound_queue: Arc::new(Mutex::new(VecDeque::new())),
            outbound_queue: Arc::new(Mutex::new(VecDeque::new())),

        }
    }

}
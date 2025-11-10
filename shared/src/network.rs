use serde::{Serialize, Deserialize};
use bincode;



/// Helper small wrappers so you don't need to import bincode directly everywhere.
pub fn serialize<T: Serialize>(msg: &T) -> Vec<u8> {
    bincode::serialize(msg).expect("serialize failed")
}

pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Option<T> {
    bincode::deserialize(data).ok()
}

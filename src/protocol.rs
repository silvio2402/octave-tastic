use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    PlaySound(PlaySound),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaySound {
    pub timestamp: u64,
    pub sound_source: String,
}

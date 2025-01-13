use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    PlaySound(PlaySound),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaySound {
    pub timestamp: u128,
    pub channels: u16,
    pub sample_rate: u32,
    pub sound_data: Vec<i16>,
}

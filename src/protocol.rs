use serde::{Deserialize, Serialize};

// Importing necessary traits from serde for serialization and deserialization

// Define an enum called Message which can hold different types of messages
#[derive(Serialize, Deserialize, Debug)]
// Automatically generate code for serialization, deserialization, and debugging
pub enum Message {
    PlaySound(PlaySound),
    // A variant of the Message enum that holds a PlaySound struct
}

// Define a struct called PlaySound which represents a sound to be played
#[derive(Serialize, Deserialize, Debug)]
// Automatically generate code for serialization, deserialization, and debugging
pub struct PlaySound {
    pub timestamp: u128,
    // The timestamp when the sound should be played
    pub channels: u16,
    // The number of audio channels (e.g., 2 for stereo)
    pub sample_rate: u32,
    // The sample rate of the audio (e.g., 44100 for 44.1 kHz)
    pub sound_data: Vec<i16>,
    // The actual sound data as a vector of 16-bit integers
}

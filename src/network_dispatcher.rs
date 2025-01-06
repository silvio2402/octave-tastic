use std::{io::Write, net::TcpStream};

use crate::protocol::{Message, PlaySound};

pub struct NetworkDispatcher {}

impl NetworkDispatcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dispatch_sound(&self, addr: String, timestamp: u128, sound_source: String) -> std::io::Result<()> {
        let msg = Message::PlaySound(PlaySound {
            timestamp: timestamp,
            sound_source: sound_source,
        });
        let mut sock = TcpStream::connect(addr).expect("Failed to connect");
        let buf = bincode::serialize(&msg).expect("Failed to serialize");
        sock.write_all(&buf)
    }
}

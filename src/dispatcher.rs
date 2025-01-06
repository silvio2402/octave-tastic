use std::io::Write;
use std::net::TcpStream;
use std::time::SystemTime;

use crate::protocol::{Message, PlaySound};

pub struct Dispatcher;

impl Dispatcher {
    pub fn handle_dispatch_sample() {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let timestamp = now + 10000000;
        let addr = "localhost:3000".to_owned();
        let msg = Message::PlaySound(PlaySound {
            timestamp: timestamp,
            sound_source: "augh.mp3".to_owned(),
        });
        let mut sock = TcpStream::connect(addr).expect("Failed to connect");
        let buf = bincode::serialize(&msg).expect("Failed to serialize");
        sock.write_all(&buf).unwrap();
    }
}

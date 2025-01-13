use std::io::{BufReader, Read};
use std::net::TcpStream;
use std::time::SystemTime;
use std::{fs::File, io::Write};

use crate::protocol::{Message, PlaySound};

pub struct Dispatcher;

impl Dispatcher {
    fn load_sound(path: String) -> Vec<u8> {
        let file = BufReader::new(File::open(path).unwrap());
        let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();
        data
    }

    pub fn handle_dispatch_sample(addrs: Vec<String>, sound_path: String) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let timestamp = now + 10000000;
        let sound_data = Dispatcher::load_sound(sound_path);
        let msg = Message::PlaySound(PlaySound {
            timestamp: timestamp,
            sound_data: sound_data,
        });
        for addr in addrs {
            if addr.is_empty() {
                continue;
                
            } else {
                let mut sock = TcpStream::connect(addr).expect("Failed to connect");
                let buf = bincode::serialize(&msg).expect("Failed to serialize");
                sock.write_all(&buf).unwrap();
            }
            
        }
        
    }
}

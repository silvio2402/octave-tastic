use std::{io::Read, net::{TcpListener, TcpStream}};

use crate::protocol::{Message, PlaySound};

pub struct NetworkHandler {}

impl NetworkHandler {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_client(&self, mut stream: TcpStream) {
        println!("Handling client: {:?}", stream);
        let mut buf: Vec<u8> = Vec::new();
        stream.read_to_end(buf.as_mut()).expect("Could not read from stream");
        let message = bincode::deserialize::<Message>(&buf).expect("Could not deserialize message");
        match message {
            Message::PlaySound(play_sound) => {
                let PlaySound { sound_source, timestamp } = play_sound;
                println!("Playing sound: {:?} at {:?}", sound_source, timestamp);
            }
        }
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("Could not bind to address");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

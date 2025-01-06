use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    protocol::{Message, PlaySound},
    sound_engine::SoundEngine,
};

pub struct NetworkHandler {
    sound_engine: Arc<SoundEngine>,
}

impl NetworkHandler {
    pub fn new(sound_engine: Arc<SoundEngine>) -> Self {
        Self { sound_engine }
    }

    fn handle_scheduled_sound(&self, play_msg: PlaySound) {
        let PlaySound {
            sound_source,
            timestamp,
        } = play_msg;
        println!("Scheduled sound: {:?} at {:?}", sound_source, timestamp);
        let now_micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Could not get time")
            .as_micros();
        let wait = timestamp as i128 - now_micros as i128;
        if wait <= 0 {
            println!("Too late to play sound");
            return;
        }
        sleep(Duration::from_micros(wait as u64));
        println!("Playing sound: {:?}", sound_source);
        self.sound_engine.play_sound(sound_source);
    }

    fn handle_client(&self, mut stream: TcpStream) {
        println!("Handling client: {:?}", stream);
        let mut buf: Vec<u8> = Vec::new();
        stream
            .read_to_end(buf.as_mut())
            .expect("Could not read from stream");
        let message = bincode::deserialize::<Message>(&buf).expect("Could not deserialize message");
        match message {
            Message::PlaySound(play_sound) => {
                self.handle_scheduled_sound(play_sound);
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

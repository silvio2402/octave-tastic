use crate::protocol::Message;
use crate::sound_scheduler::SoundScheduler;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct ClientHandler;

impl ClientHandler {
    fn handle_client(mut stream: TcpStream) {
        println!("Handling client: {:?}", stream);
        loop {
            match bincode::deserialize_from::<&mut TcpStream, Message>(&mut stream) {
                Ok(message) => {
                    match message {
                        Message::PlaySound(play_sound) => {
                            SoundScheduler::handle_scheduled_sound(play_sound);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error deserializing message: {}", e);
                    break;
                }
            }
        }
    }

    pub fn listen() {
        let listener = TcpListener::bind("0.0.0.0:3000").expect("Could not bind to address");

        for stream in listener.incoming() {
            thread::spawn(move || match stream {
                Ok(stream) => {
                    ClientHandler::handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            });
        }
    }
}

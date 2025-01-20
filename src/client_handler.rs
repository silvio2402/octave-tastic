use crate::protocol::Message;
use crate::sound_scheduler::SoundScheduler;
use std::net::UdpSocket;

pub struct ClientHandler;

impl ClientHandler {
    fn handle_message(msg: Message) {
        match msg {
            Message::PlaySound(play_sound) => {
                SoundScheduler::handle_scheduled_sound(play_sound);
            }
        }
    }

    pub fn listen() {
        let socket = UdpSocket::bind("0.0.0.0:3000").expect("Could not bind to address");

        println!("Listening on {}", socket.local_addr().unwrap());

        loop {
            let mut data = [0; 1_048_576];
            let (amt, src) = socket.recv_from(&mut data).expect("Didn't receive data");
            println!("Received {} bytes from {}", amt, src);
            let data = &mut data[..amt];
            let msg = bincode::deserialize::<Message>(data).expect("Failed to deserialize");
            ClientHandler::handle_message(msg);
        }
    }
}

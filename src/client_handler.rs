use crate::protocol::Message;
use crate::sound_scheduler::SoundScheduler;
use std::net::UdpSocket;

pub struct BindError {
    pub message: String,
}

type Result<T> = std::result::Result<T, BindError>;

pub struct ClientHandler;

impl ClientHandler {
    fn handle_message(msg: Message) {
        match msg {
            Message::PlaySound(play_sound) => {
                SoundScheduler::handle_scheduled_sound(play_sound);
            }
        }
    }

    pub fn listen(port: u16) -> Result<()> {
        let bind_addr = format!("0.0.0.0:{}", port);
        let socket = UdpSocket::bind(bind_addr);
        let socket = match socket {
            Ok(s) => s,
            Err(e) => {
                return Err(BindError {
                    message: e.to_string(),
                });
            }
        };

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

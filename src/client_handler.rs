use crate::protocol::Message;
use crate::sound_scheduler::SoundScheduler;
use std::net::UdpSocket;

// Custom error type for binding errors
pub struct BindError {
    pub message: String,
}

// Type alias for Result with BindError as the error type
type Result<T> = std::result::Result<T, BindError>;

// Trait for handling client messages
pub trait ClientHandler {
    // Function to handle incoming messages
    fn handle_message(msg: Message) {
        match msg {
            // Match on the message type and handle accordingly
            Message::PlaySound(play_sound) => {
                // Delegate to SoundScheduler to handle the play sound message
                SoundScheduler::handle_scheduled_sound(play_sound);
            }
        }
    }
}

// Trait for listening to incoming messages
pub trait Listener: ClientHandler {
    // Function to start listening on a given port
    fn listen(port: u16) -> Result<()>;
}

// Struct representing a network listener
pub struct NetworkListener;

// Implement ClientHandler for NetworkListener
impl ClientHandler for NetworkListener {}

// Implement Listener for NetworkListener
impl Listener for NetworkListener {
    fn listen(port: u16) -> Result<()> {
        // Create the bind address string
        let bind_addr = format!("0.0.0.0:{}", port);

        // Attempt to bind the UDP socket to the address
        let socket = UdpSocket::bind(bind_addr);
        let socket = match socket {
            Ok(s) => s,
            Err(e) => {
                // If there's an error, return a BindError
                return Err(BindError {
                    message: e.to_string(),
                });
            }
        };

        // Print the local address the socket is bound to
        println!("Listening on {}", socket.local_addr().unwrap());

        // Enter an infinite loop to listen for incoming messages
        loop {
            // Buffer to store incoming data
            let mut data = [0; 1_048_576];

            // Receive data from the socket
            let (amt, src) = socket.recv_from(&mut data).expect("Didn't receive data");

            // Print the amount of data received and the source address
            println!("Received {} bytes from {}", amt, src);

            // Slice the buffer to the actual amount of data received
            let data = &mut data[..amt];

            // Deserialize the data into a Message
            let msg = bincode::deserialize::<Message>(data).expect("Failed to deserialize");

            // Handle the deserialized message
            NetworkListener::handle_message(msg);
        }
    }
}

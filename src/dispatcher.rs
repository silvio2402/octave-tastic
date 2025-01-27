use std::fs::File;
use std::io::BufReader;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, SystemTime};

use rodio::{Decoder, Source};

use crate::protocol::{Message, PlaySound};

// Define a trait for dispatching tasks
pub trait Dispatcher {
    // Method to handle dispatching tasks to multiple addresses
    fn handle_dispatch(addrs: Vec<String>, sound_path: String);
}

// Struct for network-based dispatching
pub struct NetworkDispatcher;

impl NetworkDispatcher {
    // Load a sound file and return a Decoder
    fn load_sound(path: String) -> Decoder<BufReader<File>> {
        let file = BufReader::new(File::open(path).unwrap()); // Open the file
        Decoder::new(file).unwrap() // Decode the file
    }
}

impl Dispatcher for NetworkDispatcher {
    fn handle_dispatch(addrs: Vec<String>, sound_path: String) {
        // Get the current system time since UNIX_EPOCH
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        // Load and buffer the sound source
        let source = NetworkDispatcher::load_sound(sound_path).buffered();

        // Get sound properties
        let channels = source.channels();
        let sample_rate = source.sample_rate();
        let duration = source.total_duration().unwrap();
        let chunk_len = Duration::from_millis(200); // Length of each chunk to send

        let mut offset = Duration::from_secs(0); // Initial offset

        // Iterate over each address
        for addr in addrs {
            if addr.is_empty() {
                continue; // Skip empty addresses
            }

            let source = source.clone(); // Clone the source for each thread

            // Spawn a new thread for each address
            thread::spawn(move || {
                // Bind to a local UDP socket
                let sock = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind");
                sock.connect(addr.clone()).expect("Failed to connect"); // Connect to the address

                // Loop until the entire duration is covered
                while offset < duration {
                    // Collect sound data for the current chunk
                    let sound_data = source
                        .clone()
                        .skip_duration(offset)
                        .take_duration(chunk_len)
                        .collect::<Vec<i16>>();

                    offset += chunk_len; // Update the offset

                    // Create a PlaySound message
                    let msg = Message::PlaySound(PlaySound {
                        timestamp: (now + offset).as_micros(), // Timestamp for synchronization
                        channels: channels,
                        sample_rate: sample_rate,
                        sound_data: sound_data,
                    });

                    // Serialize the message
                    let buf = bincode::serialize(&msg).expect("Failed to serialize");
                    // Send the serialized message via UDP
                    sock.send(&buf).expect("Failed to send");
                }
            });
        }
    }
}

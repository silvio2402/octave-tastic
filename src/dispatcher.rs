use std::io::BufReader;
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, SystemTime};
use std::{fs::File, io::Write};

use rodio::{Decoder, Source};

use crate::protocol::{Message, PlaySound};

pub struct Dispatcher;

impl Dispatcher {
    fn load_sound(path: String) -> Decoder<BufReader<File>> {
        let file = BufReader::new(File::open(path).unwrap());
        Decoder::new(file).unwrap()
    }

    pub fn handle_dispatch_sample(addrs: Vec<String>, sound_path: String) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let source = Dispatcher::load_sound(sound_path).buffered();

        let channels = source.channels();
        let sample_rate = source.sample_rate();
        let duration = source.total_duration().unwrap();
        let chunk_len = Duration::from_secs(1);

        let mut offset = Duration::from_secs(0);

        for addr in addrs {
            if addr.is_empty() {
                continue;
            }

            let source = source.clone();

            thread::spawn(move || {
                let mut sock = TcpStream::connect(addr).expect("Failed to connect");

                while offset < duration {
                    let sound_data = source
                        .clone()
                        .skip_duration(offset)
                        .take_duration(chunk_len)
                        .collect::<Vec<i16>>();

                    offset += chunk_len;

                    let msg = Message::PlaySound(PlaySound {
                        timestamp: (now + offset).as_micros(),
                        channels: channels,
                        sample_rate: sample_rate,
                        sound_data: sound_data,
                    });

                    let buf = bincode::serialize(&msg).expect("Failed to serialize");
                    sock.write_all(&buf).unwrap();
                }
            });
        }
    }
}

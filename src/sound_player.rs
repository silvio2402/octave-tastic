use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

pub struct SoundPlayer;

impl SoundPlayer {
    pub fn play_sound(sound_source: String) {
        let path = format!("./audios/{}", sound_source);
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}

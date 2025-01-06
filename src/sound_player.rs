use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

pub struct SoundPlayer;

impl SoundPlayer {
    pub fn play_sound(sound_data: Vec<u8>) {
        let data = Cursor::new(sound_data);
        let source = Decoder::new(data).unwrap();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}

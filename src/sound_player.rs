use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

pub struct SoundPlayer;

impl SoundPlayer {
    pub fn play_sound(channels: u16, sample_rate: u32, sound_data: Vec<i16>) {
        // let data = Cursor::new(sound_data);
        let source = SamplesBuffer::new(channels, sample_rate, sound_data);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}

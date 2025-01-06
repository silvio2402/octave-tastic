use std::{fs::File, io::BufReader};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

pub struct SoundEngine {
    stream_handle: OutputStreamHandle,
}

impl SoundEngine {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self { stream_handle }
    }

    pub fn play_sound(&self, sound_source: String) {
        let path = format!("./audios/{}", sound_source);
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.stream_handle
            .play_raw(source.convert_samples())
            .unwrap();
    }
}

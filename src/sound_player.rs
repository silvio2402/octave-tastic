use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

// Define a struct for the SoundPlayer
pub struct SoundPlayer;

impl SoundPlayer {
    // Function to play sound given channels, sample rate, and sound data
    pub fn play_sound(channels: u16, sample_rate: u32, sound_data: Vec<i16>) {
        // Create a SamplesBuffer from the provided sound data
        let source = SamplesBuffer::new(channels, sample_rate, sound_data);

        // Try to get the default output stream and handle
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Create a new Sink (audio output) using the stream handle
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Append the sound source to the sink
        sink.append(source);

        // Block the current thread until the sound finishes playing
        sink.sleep_until_end();
    }
}

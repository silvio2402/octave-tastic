use crate::protocol::PlaySound;
use crate::sound_player::SoundPlayer;
use std::thread::{self, sleep};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Define a struct for the SoundScheduler
pub struct SoundScheduler;

impl SoundScheduler {
    // Function to handle scheduled sound playback
    pub fn handle_scheduled_sound(play_msg: PlaySound) {
        // Spawn a new thread to handle the sound playback
        thread::spawn(move || {
            // Destructure the PlaySound message
            let PlaySound {
                channels,
                sample_rate,
                sound_data,
                timestamp,
            } = play_msg;

            // Print the length of the sound data and the scheduled timestamp
            println!(
                "Scheduled sound: len={:?} at {:?}",
                sound_data.len(),
                timestamp
            );

            // Get the current time in microseconds since UNIX_EPOCH
            let now_micros = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Could not get time")
                .as_micros();

            // Calculate the wait time in microseconds
            let wait = timestamp as i128 - now_micros as i128;

            // If the wait time is less than or equal to zero, it's too late to play the sound
            if wait <= 0 {
                println!("Too late to play sound");
                return;
            }

            // Sleep for the calculated wait time
            sleep(Duration::from_micros(wait as u64));

            // Print a message indicating that the sound is being played
            println!("Playing sound...");

            // Play the sound using the SoundPlayer
            SoundPlayer::play_sound(channels, sample_rate, sound_data);
        });
    }
}

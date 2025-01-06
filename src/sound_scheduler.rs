use crate::protocol::PlaySound;
use crate::sound_player::SoundPlayer;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct SoundScheduler;

impl SoundScheduler {
    pub fn handle_scheduled_sound(play_msg: PlaySound) {
        let PlaySound {
            sound_source,
            timestamp,
        } = play_msg;
        println!("Scheduled sound: {:?} at {:?}", sound_source, timestamp);
        let now_micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Could not get time")
            .as_micros();
        let wait = timestamp as i128 - now_micros as i128;
        if wait <= 0 {
            println!("Too late to play sound");
            return;
        }
        sleep(Duration::from_micros(wait as u64));
        println!("Playing sound: {:?}", sound_source);
        SoundPlayer::play_sound(sound_source);
    }
}

use std::{env, thread};
use std::time::Duration;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    track::TrackBuilder,
    effect::filter::FilterBuilder,
};
use kira::effect::filter::FilterMode;

fn main() {

    let mut sound_file_path = env::current_dir().unwrap().display().to_string() + "/Audio_asset/sound.ogg";

    println!("Sound File: {}", sound_file_path);

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Sound Manager fail to setup");
    // Create a mixer sub-track with a filter.
    let mut track = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(FilterBuilder::new().cutoff(100.0).mix(1.0).mode(FilterMode::LowPass));
        builder
    }).expect("Track setup failure");

    

    // Play the sound on the track.
    let sound_data = StaticSoundData::from_file(sound_file_path).expect("Can not load sound data");
    track.play(sound_data).expect("Can not play sound");

    thread::sleep(Duration::from_secs(5));
}
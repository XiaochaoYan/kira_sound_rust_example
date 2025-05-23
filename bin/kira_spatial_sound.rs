use std::{env, thread};
use std::time::Duration;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    track::TrackBuilder,
    effect::filter::FilterBuilder,
};
use kira::effect::filter::FilterMode;

use kira::{
    effect::reverb::ReverbBuilder,
    Easing, Mapping, Value,
    Mix,
};
use kira::track::SpatialTrackBuilder;

fn main() {

    // let mut sound_file_path = env::current_dir().unwrap().display().to_string() + "/Audio_asset/sound.ogg";
    // 
    // println!("Sound File: {}", sound_file_path);
    // 
    // let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Sound Manager fail to setup");
    // // Create a mixer sub-track with a filter.
    // let mut track = manager.add_sub_track({
    //     let mut builder = TrackBuilder::new();
    //     builder.add_effect(FilterBuilder::new().cutoff(100.0).mix(1.0).mode(FilterMode::LowPass));
    //     builder
    // }).expect("Track setup failure");
    // // Play the sound on the track.
    // let sound_data = StaticSoundData::from_file(sound_file_path).expect("Can not load sound data");
    // track.play(sound_data).expect("Can not play sound");

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Sound Manager fail to setup");
    let listener = manager.add_listener(glam::Vec3::ZERO, glam::Quat::IDENTITY).expect("Sound listener fail to setup");

    let mut spatial_track_right = manager.add_spatial_sub_track(
        &listener,
        glam::vec3(10.0, 0.0, 0.0), // track position
        SpatialTrackBuilder::new(),
    ).expect("Spatial track failure");

    let mut spatial_track_left = manager.add_spatial_sub_track(
        &listener,
        glam::vec3(-10.0, 0.0, 0.0), // track position
        SpatialTrackBuilder::new(),
    ).expect("Spatial track failure");

    let mut sound_file_path = env::current_dir().unwrap().display().to_string() + "/Audio_asset/drums.ogg";
    let sound_data = StaticSoundData::from_file(sound_file_path).expect("Can not load sound data");
    
    spatial_track_right.play(sound_data.clone()).expect("Can not play sound");
    thread::sleep(Duration::from_secs(5));

    spatial_track_left.play(sound_data.clone()).expect("Can not play sound");
    thread::sleep(Duration::from_secs(5));
}
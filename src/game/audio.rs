use crate::game::{AudioEvent, GameSounds};
use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundMusic;

pub fn play_audio_events(
    mut audio_events: EventReader<AudioEvent>,
    game_sounds: Res<GameSounds>,
    mut commands: Commands,
) {
    for event in audio_events.read() {
        let audio_source = match event {
            AudioEvent::Wing => game_sounds.wing.clone(),
            AudioEvent::Point => game_sounds.point.clone(),
            AudioEvent::Hit => game_sounds.hit.clone(),
            AudioEvent::Die => game_sounds.die.clone(),
        };

        commands.spawn((AudioPlayer::new(audio_source), PlaybackSettings::ONCE));
    }
}

// Background music system
pub fn play_background_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/random_game_music.ogg")),
        PlaybackSettings::LOOP,
        BackgroundMusic,
    ));
}

// Stop background music when entering game over
pub fn stop_background_music(
    mut commands: Commands,
    background_music_query: Query<Entity, With<BackgroundMusic>>,
) {
    for entity in &background_music_query {
        commands.entity(entity).despawn();
    }
}

use bevy::prelude::*;
use crate::game::{AudioEvent, GameSounds};

pub fn play_audio_events(
    mut audio_events: EventReader<AudioEvent>,
    game_sounds: Res<GameSounds>,
    mut commands: Commands,
) {
    for event in audio_events.read() {
        match event {
            AudioEvent::Wing => {
                commands.spawn((
                    AudioPlayer::new(game_sounds.wing.clone()),
                    PlaybackSettings::ONCE,
                ));
            }
            AudioEvent::Point => {
                commands.spawn((
                    AudioPlayer::new(game_sounds.point.clone()),
                    PlaybackSettings::ONCE,
                ));
            }
            AudioEvent::Hit => {
                commands.spawn((
                    AudioPlayer::new(game_sounds.hit.clone()),
                    PlaybackSettings::ONCE,
                ));
            }
            AudioEvent::Die => {
                commands.spawn((
                    AudioPlayer::new(game_sounds.die.clone()),
                    PlaybackSettings::ONCE,
                ));
            }
        }
    }
} 
use bevy::prelude::*;
use crate::game::{
    constants::*,
    components::*,
    events::AudioEvent,
};

pub fn update_score(
    player_query: Single<&Transform, With<Player>>,
    pipe_pairs_query: Query<&Transform, With<PipePair>>,
    mut score: ResMut<Score>,
    score_text_query: Single<&mut Text, With<ScoreText>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    let player = player_query.into_inner();
    let mut score_text = score_text_query.into_inner();

    for transform in &pipe_pairs_query {
        let threshold = transform.translation.x + PIPE_WIDTH / 2.0;

        if player.translation.x == threshold {
            score.0 += 1;
            *score_text = Text(score.0.to_string());
            // Send point sound event
            audio_events.write(AudioEvent::Point);
        }
    }
} 
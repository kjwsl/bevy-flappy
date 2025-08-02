use bevy::prelude::*;
use crate::game::{
    constants::*,
    components::*,
    events::AudioEvent,
};

pub fn update_score(
    player_query: Single<&Transform, With<Player>>,
    mut pipe_pairs_query: Query<(&Transform, &mut PipePair)>,
    mut score: ResMut<Score>,
    mut difficulty: ResMut<Difficulty>,
    score_text_query: Single<&mut Text, With<ScoreText>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    let player = player_query.into_inner();
    let mut score_text = score_text_query.into_inner();

    for (transform, mut pipe_pair) in &mut pipe_pairs_query {
        let threshold = transform.translation.x + PIPE_WIDTH / 2.0;

        // Check if player has passed the pipe and we haven't scored it yet
        if player.translation.x > threshold && !pipe_pair.scored {
            pipe_pair.scored = true;
            score.0 += 1;
            *score_text = Text(score.0.to_string());
            
            // Update difficulty based on new score
            difficulty.update_difficulty(score.0);
            
            // Send point sound event
            audio_events.write(AudioEvent::Point);
        }
    }
} 
use std::f32::consts::PI;
use bevy::prelude::*;
use rand::Rng;
use crate::game::{
    constants::*,
    components::*,
};

pub fn generate_pipes(
    mut commands: Commands,
    pipe_textures: Res<PipeTextures>,
    mut interval: ResMut<PipeInterval>,
    difficulty: Res<Difficulty>,
    time: Res<Time>,
    root_query: Query<Entity, With<GameWorld>>,
) {
    let root = root_query.single().expect("Game scene not found");

    interval.0.tick(time.delta());
    if interval.0.finished() {
        interval.0.reset();
        
        // Update the interval timer with current difficulty for next spawn
        interval.update_interval(INITIAL_PIPE_INTERVAL, difficulty.spawn_interval_multiplier);

        // Apply difficulty multiplier to pipe gap (starts large, gets smaller)
        let base_gap = rand::rng().random_range(MIN_PIPE_GAP..MAX_PIPE_GAP);
        let pipe_gap = base_gap * difficulty.pipe_gap_multiplier;

        // Ensure pipe gap doesn't get too small to avoid invalid ranges
        let min_gap = 80.0; // Increased minimum safe gap
        let safe_pipe_gap = pipe_gap.max(min_gap);

        // Calculate the valid range for pipe positioning
        let min_y = -BG_IMG_DIMENSIONS.1 / 2.0 + safe_pipe_gap / 2.0 + PIPE_LEGROOM;
        let max_y = BG_IMG_DIMENSIONS.1 / 2.0 - safe_pipe_gap / 2.0 - PIPE_LEGROOM;
        
        // Ensure the range is valid
        let new_y = if min_y >= max_y {
            // If range is invalid, use a default position
            0.0
        } else {
            rand::rng().random_range(min_y..max_y)
        };

        let pipe_offset = safe_pipe_gap / 2.0 + PIPE_HEIGHT / 2.0;
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                PipePair::default(),
                Transform::from_xyz(BG_IMG_DIMENSIONS.0 + PIPE_WIDTH / 2.0, new_y, Z_POS_PIPE),
                Visibility::Visible,
                children![
                    (
                        Pipe,
                        Sprite {
                            image: pipe_textures.green_pipe.clone(),
                            ..default()
                        },
                        Transform::from_xyz(0., -pipe_offset, 0.,),
                        Collider,
                    ),
                    (
                        Pipe,
                        Sprite {
                            image: pipe_textures.green_pipe.clone(),
                            ..default()
                        },
                        Transform {
                            translation: Vec3::new(0., pipe_offset, 0.,),
                            rotation: Quat::from_rotation_x(PI),
                            ..default()
                        },
                        Collider,
                    )
                ],
            ));
        });
    }
}

pub fn move_pipes(
    mut query: Query<(&mut Transform, &PipePair)>,
    difficulty: Res<Difficulty>,
) {
    for (mut transform, _) in &mut query {
        // Apply difficulty multiplier to pipe speed
        transform.translation.x -= PIPE_SPEED * difficulty.pipe_speed_multiplier;
    }
}

pub fn destroy_pipes(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    let threshold = -GAME_DIMENSIONS.0 / 2.0 - PIPE_WIDTH / 2.0;
    for (entity, transform) in &query {
        if transform.translation.x < threshold {
            commands.entity(entity).despawn();
        }
    }
} 
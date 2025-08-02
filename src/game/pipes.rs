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
    time: Res<Time>,
    root_query: Query<Entity, With<GameWorld>>,
) {
    let root = root_query.single().expect("Game scene not found");

    interval.0.tick(time.delta());
    if interval.0.finished() {
        interval.0.reset();

        let pipe_gap = rand::rng().random_range(MIN_PIPE_GAP..MAX_PIPE_GAP);

        let new_y = rand::rng().random_range(
            (-BG_IMG_DIMENSIONS.1 / 2.0 + pipe_gap / 2.0 + PIPE_LEGROOM)
                ..(BG_IMG_DIMENSIONS.1 / 2.0 - pipe_gap / 2.0 - PIPE_LEGROOM),
        );

        let pipe_offset = pipe_gap / 2.0 + PIPE_HEIGHT / 2.0;
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                PipePair,
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

pub fn move_pipes(mut query: Query<(&mut Transform, &PipePair)>) {
    for (mut transform, _) in &mut query {
        transform.translation.x -= PIPE_SPEED;
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
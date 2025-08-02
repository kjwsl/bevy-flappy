use bevy::prelude::*;
use crate::game::constants::*;

pub fn check_collision(player_transform: &GlobalTransform, pipe_transform: &GlobalTransform) -> bool {
    // Simple AABB collision detection using world coordinates
    let player_left = player_transform.translation().x - PLAYER_COLLISION_WIDTH / 2.0;
    let player_right = player_transform.translation().x + PLAYER_COLLISION_WIDTH / 2.0;
    let player_top = player_transform.translation().y + PLAYER_COLLISION_HEIGHT / 2.0;
    let player_bottom = player_transform.translation().y - PLAYER_COLLISION_HEIGHT / 2.0;

    let pipe_left = pipe_transform.translation().x - PIPE_COLLISION_WIDTH / 2.0;
    let pipe_right = pipe_transform.translation().x + PIPE_COLLISION_WIDTH / 2.0;
    let pipe_top = pipe_transform.translation().y + PIPE_COLLISION_HEIGHT / 2.0;
    let pipe_bottom = pipe_transform.translation().y - PIPE_COLLISION_HEIGHT / 2.0;

    // Check if rectangles overlap
    player_left < pipe_right
        && player_right > pipe_left
        && player_top > pipe_bottom
        && player_bottom < pipe_top
} 
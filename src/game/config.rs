use bevy::prelude::*;
use crate::game::constants::*;

#[derive(Resource, Clone)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub pipes: PipeConfig,
    pub audio: AudioConfig,
    pub ui: UiConfig,
}

#[derive(Clone)]
pub struct PlayerConfig {
    pub initial_position: Vec2,
    pub jump_impulse: f32,
    pub gravity: f32,
    pub max_fall_speed: f32,
    pub max_rotation: f32,
    pub collision_size: Vec2,
}

#[derive(Clone)]
pub struct PipeConfig {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub min_gap: f32,
    pub max_gap: f32,
    pub legroom: f32,
    pub spawn_interval: f32,
    pub collision_size: Vec2,
}

#[derive(Clone)]
pub struct AudioConfig {
    pub wing_sound: String,
    pub point_sound: String,
    pub hit_sound: String,
    pub die_sound: String,
}

#[derive(Clone)]
pub struct UiConfig {
    pub score_font_size: f32,
    pub game_over_font_size: f32,
    pub button_colors: ButtonColors,
}

#[derive(Clone)]
pub struct ButtonColors {
    pub idle: Color,
    pub hover: Color,
    pub pressed: Color,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player: PlayerConfig {
                initial_position: Vec2::new(-150.0, 70.0),
                jump_impulse: JUMP_IMPULSE,
                gravity: GRAVITY,
                max_fall_speed: MAX_FALL_SPEED,
                max_rotation: MAX_PLAYER_ROTATION,
                collision_size: Vec2::new(PLAYER_COLLISION_WIDTH, PLAYER_COLLISION_HEIGHT),
            },
            pipes: PipeConfig {
                width: PIPE_WIDTH,
                height: PIPE_HEIGHT,
                speed: PIPE_SPEED,
                min_gap: MIN_PIPE_GAP,
                max_gap: MAX_PIPE_GAP,
                legroom: PIPE_LEGROOM,
                spawn_interval: INITIAL_PIPE_INTERVAL,
                collision_size: Vec2::new(PIPE_COLLISION_WIDTH, PIPE_COLLISION_HEIGHT),
            },
            audio: AudioConfig {
                wing_sound: "audio/wing.ogg".to_string(),
                point_sound: "audio/point.ogg".to_string(),
                hit_sound: "audio/hit.ogg".to_string(),
                die_sound: "audio/die.ogg".to_string(),
            },
            ui: UiConfig {
                score_font_size: 50.0,
                game_over_font_size: 70.0,
                button_colors: ButtonColors {
                    idle: BUTTON_COLOR_IDLE,
                    hover: BUTTON_COLOR_HOVER,
                    pressed: BUTTON_COLOR_PRESSED,
                },
            },
        }
    }
} 
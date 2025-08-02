use bevy::prelude::*;
use bevy_flappy_macros::hex_to_color;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
    Settings,
}

pub const GAME_DIMENSIONS: (f32, f32) = (BG_IMG_DIMENSIONS.0 * 2.0, BG_IMG_DIMENSIONS.1);

pub const BUTTON_COLOR_IDLE: Color = hex_to_color!("#E5E5E5");
pub const BUTTON_COLOR_HOVER: Color = hex_to_color!("#D3D3D3");
pub const BUTTON_COLOR_PRESSED: Color = hex_to_color!("#A9A9A9");

pub const BG_IMG_DIMENSIONS: (f32, f32) = (288.0, 512.0);
pub const BG_SPRITE_PATH: &str = "sprites/background-day.png";
pub const PLATFORM_SPRITE_PATH: &str = "sprites/base.png";

pub const Z_POS_BG: f32 = -10.0;
pub const Z_POS_PLATFORM: f32 = -3.0;
pub const Z_POS_PIPE: f32 = -4.0;
pub const Z_POS_PLAYER: f32 = 10.0;

pub const MAX_PLAYER_ROTATION: f32 = 25.0;

pub const BG_SPEED: f32 = 0.2;
pub const PLATFORM_SPEED: f32 = 1.0;
pub const PIPE_SPEED: f32 = 1.0;

pub const GRAVITY: f32 = -700.0;
pub const JUMP_IMPULSE: f32 = 300.0;
pub const MAX_FALL_SPEED: f32 = -700.0;
pub const MAX_HEIGHT: f32 = GAME_DIMENSIONS.1 / 2.0;

pub const PIPE_WIDTH: f32 = 52.0;
pub const PIPE_HEIGHT: f32 = 320.0;
pub const PIPE_LEGROOM: f32 = 100.0;
pub const MAX_PIPE_GAP: f32 = 200.0;
pub const MIN_PIPE_GAP: f32 = 100.0;

pub const INITIAL_PIPE_INTERVAL: f32 = 3.0;

// Collision detection constants
pub const PLAYER_COLLISION_WIDTH: f32 = 24.0;
pub const PLAYER_COLLISION_HEIGHT: f32 = 24.0;
pub const PIPE_COLLISION_WIDTH: f32 = 52.0;
pub const PIPE_COLLISION_HEIGHT: f32 = 320.0; 
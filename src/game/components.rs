use bevy::prelude::*;

#[derive(Component)]
pub struct GameWorld;

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct GameOverLayer;

#[derive(Component)]
pub struct BackgroundImage;

#[derive(Component)]
pub struct PlatformImage;

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub f32);

#[derive(Component, Default)]
pub struct Collider;

#[derive(Resource, Clone)]
pub struct BirdTextures {
    pub up: Handle<Image>,
    pub mid: Handle<Image>,
    pub down: Handle<Image>,
}

#[derive(Resource, Clone, Default)]
pub struct Score(pub u32);

#[derive(Component, Clone)]
#[require(Text)]
pub struct ScoreText;

#[derive(Component, Clone)]
#[require(Sprite, Transform, Collider)]
pub struct Pipe;

#[derive(Component, Clone)]
pub struct PipePair {
    pub scored: bool,
}

impl Default for PipePair {
    fn default() -> Self {
        Self { scored: false }
    }
}

#[derive(Resource, Clone)]
pub struct PipeTextures {
    pub green_pipe: Handle<Image>,
    pub red_pipe: Handle<Image>,
}

#[derive(Resource)]
pub struct PipeInterval(pub Timer);

impl PipeInterval {
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

impl Default for PipeInterval {
    fn default() -> Self {
        use crate::game::constants::INITIAL_PIPE_INTERVAL;
        Self(Timer::from_seconds(
            INITIAL_PIPE_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

impl PipeInterval {
    pub fn update_interval(&mut self, base_interval: f32, multiplier: f32) {
        self.0 = Timer::from_seconds(base_interval * multiplier, TimerMode::Repeating);
    }
}

#[derive(Resource)]
pub struct GameSounds {
    pub wing: Handle<AudioSource>,
    pub point: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub die: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct Difficulty {
    pub current_level: u32,
    pub pipe_speed_multiplier: f32,
    pub pipe_gap_multiplier: f32,
    pub spawn_interval_multiplier: f32,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self {
            current_level: 0,
            pipe_speed_multiplier: 1.0,
            pipe_gap_multiplier: 1.0,
            spawn_interval_multiplier: 1.0,
        }
    }
}

impl Difficulty {
    pub fn update_difficulty(&mut self, score: u32) {
        let new_level = score / 15; // Increase difficulty every 15 points (much slower)
        
        if new_level != self.current_level {
            self.current_level = new_level;
            
            // Increase pipe speed (max 2.0x, very gradual)
            self.pipe_speed_multiplier = (1.0 + (self.current_level as f32 * 0.02)).min(2.0);
            
            // Decrease pipe gap (starts at 1.0x, goes down to 0.7x, very gradual)
            self.pipe_gap_multiplier = (1.0 - (self.current_level as f32 * 0.005)).max(0.7);
            
            // Decrease spawn interval (starts at 1.0x, goes down to 0.6x, very gradual)
            self.spawn_interval_multiplier = (1.0 - (self.current_level as f32 * 0.008)).max(0.6);
        }
    }
}

#[derive(Component)]
pub enum GameOverMenuButton {
    Retry,
    MainMenu,
} 
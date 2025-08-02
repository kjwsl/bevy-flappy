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
pub struct PipePair;

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

#[derive(Resource)]
pub struct GameSounds {
    pub wing: Handle<AudioSource>,
    pub point: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub die: Handle<AudioSource>,
}

#[derive(Component)]
pub enum GameOverMenuButton {
    Retry,
    MainMenu,
} 
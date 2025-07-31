use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
    Settings,
}

pub const BG_IMG_DIMENSIONS: (f32, f32) = (288.0, 512.0);
pub const BG_SPRITE_PATH: &str = "sprites/background-day.png";
pub const PLATFORM_SPRITE_PATH: &str = "sprites/base.png";

const Z_POS_BG: f32 = -10.0;
const Z_POS_PLATFORM: f32 = -3.0;
const Z_POS_PLAYER: f32 = 10.0;

const BG_SPEED: f32 = 0.4;
const PLATFORM_SPEED: f32 = 1.0;

const GRAVITY: f32 = -700.0;
const JUMP_IMPULSE: f32 = 300.0;
const MAX_FALL_SPEED: f32 = -700.0;

pub struct GamePlugin;

#[derive(Component)]
pub struct GameScene;

#[derive(Component)]
pub struct BackgroundImage;

#[derive(Component)]
pub struct PlatformImage;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub f32);

#[derive(Resource)]
struct BirdTextures {
    up: Handle<Image>,
    mid: Handle<Image>,
    down: Handle<Image>,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (move_bg, apply_gravity, handle_jump_input, detect_gameover)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures = BirdTextures {
        up: asset_server.load("sprites/yellowbird-upflap.png"),
        mid: asset_server.load("sprites/yellowbird-midflap.png"),
        down: asset_server.load("sprites/yellowbird-downflap.png"),
    };
    let root = commands.spawn((GameScene, Transform::default())).id();

    commands.entity(root).with_children(|parent| {
        // Player
        parent.spawn((
            Sprite {
                image: textures.up.clone(),
                ..default()
            },
            Transform::from_xyz(-150., 0., Z_POS_PLAYER),
            Velocity(0.),
            Player,
        ));

        // Background & Platform
        for i in -1..=2 {
            parent.spawn((
                Sprite {
                    image: asset_server.load(BG_SPRITE_PATH),
                    custom_size: Some(Vec2::new(BG_IMG_DIMENSIONS.0, BG_IMG_DIMENSIONS.1)),
                    ..default()
                },
                Transform::from_xyz(i as f32 * BG_IMG_DIMENSIONS.0, 0., Z_POS_BG),
                BackgroundImage,
            ));
            parent.spawn((
                Sprite {
                    image: asset_server.load(PLATFORM_SPRITE_PATH),
                    ..default()
                },
                Transform::from_xyz(i as f32 * BG_IMG_DIMENSIONS.0, -250., Z_POS_PLATFORM),
                PlatformImage,
            ));
        }
    });

    commands.insert_resource(textures);
}

fn detect_gameover(
    player_query: Query<&Transform, With<Player>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(transform) = player_query.single() {
        if transform.translation.y < -BG_IMG_DIMENSIONS.1 / 2.0 - 30.0 {
            app_state.set(AppState::GameOver);
        }
    }
}

fn move_bg(
    mut bg_query: Query<&mut Transform, With<BackgroundImage>>,
    mut platform_query: Query<&mut Transform, (With<PlatformImage>, Without<BackgroundImage>)>,
) {
    for mut transform in &mut bg_query {
        transform.translation.x -= BG_SPEED;

        if transform.translation.x < -BG_IMG_DIMENSIONS.0 * 1.5 {
            transform.translation.x = BG_IMG_DIMENSIONS.0 * 1.5;
        }
    }

    for mut transform in &mut platform_query {
        transform.translation.x -= PLATFORM_SPEED;

        if transform.translation.x < -BG_IMG_DIMENSIONS.0 * 1.5 {
            transform.translation.x = BG_IMG_DIMENSIONS.0 * 1.5;
        }
    }
}

fn apply_gravity(
    textures: Res<BirdTextures>,
    time: Res<Time>,
    mut player_query: Query<(&mut Sprite, &mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut sprite, mut transform, mut velocity) in &mut player_query {
        **velocity += GRAVITY * time.delta_secs();
        **velocity = (**velocity).max(MAX_FALL_SPEED);

        transform.translation.y += **velocity * time.delta_secs();

        sprite.image = if **velocity > 150.0 {
            textures.up.clone()
        } else if **velocity < -150.0 {
            textures.down.clone()
        } else {
            textures.mid.clone()
        };
    }
}

fn handle_jump_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut velocity in &mut player_query {
            *velocity = Velocity(JUMP_IMPULSE)
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<GameScene>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

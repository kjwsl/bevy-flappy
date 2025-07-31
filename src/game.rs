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
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures = BirdTextures {
        up: asset_server.load("sprites/yellowbird-upflap.png"),
        mid: asset_server.load("sprites/yellowbird-midflap.png"),
        down: asset_server.load("sprites/yellowbird-downflap.png"),
    };
    spawn_player(&mut commands, &textures);
    create_bg(&mut commands, &asset_server);

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

fn spawn_player(commands: &mut Commands, bird_textures: &BirdTextures) {
    let player_asset = bird_textures.mid.clone();
    commands.spawn((
        Sprite {
            image: player_asset,
            ..default()
        },
        Transform {
            translation: Vec3::new(-150.0, 0.0, Z_POS_PLAYER),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        Velocity(0.0),
        GlobalTransform::default(),
        Player,
    ));
}

fn create_bg(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let bg_handle = asset_server.load(BG_SPRITE_PATH);
    let base_handle = asset_server.load(PLATFORM_SPRITE_PATH);

    for i in -1..=2 {
        commands.spawn((
            Sprite {
                image: bg_handle.clone(),
                custom_size: Some(Vec2::new(BG_IMG_DIMENSIONS.0, BG_IMG_DIMENSIONS.1)),
                ..default()
            },
            Transform {
                translation: Vec3::new(i as f32 * BG_IMG_DIMENSIONS.0, 0.0, Z_POS_BG),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            GlobalTransform::default(),
            BackgroundImage,
        ));

        commands.spawn((
            Sprite {
                image: base_handle.clone(),
                ..default()
            },
            Transform {
                translation: Vec3::new(i as f32 * BG_IMG_DIMENSIONS.0, -250.0, Z_POS_PLATFORM),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            GlobalTransform::default(),
            PlatformImage,
        ));
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

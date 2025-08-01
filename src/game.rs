use bevy::{prelude::*, text::LineHeight};
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

const Z_POS_BG: f32 = -10.0;
const Z_POS_PLATFORM: f32 = -3.0;
const Z_POS_PLAYER: f32 = 10.0;

const BG_SPEED: f32 = 0.2;
const PLATFORM_SPEED: f32 = 1.0;

const GRAVITY: f32 = -700.0;
const JUMP_IMPULSE: f32 = 300.0;
const MAX_FALL_SPEED: f32 = -700.0;
const MAX_HEIGHT: f32 = GAME_DIMENSIONS.1 / 2.0;

#[derive(Component)]
pub enum GameOverMenuButton {
    Retry,
    MainMenu,
}

pub struct GamePlugin;

#[derive(Component)]
pub struct GameScene;

#[derive(Component)]
pub struct GameOverLayer;

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
            .add_systems(OnEnter(AppState::GameOver), setup_gameover)
            .add_systems(
                Update,
                (handle_gameover_menu_button).run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures = BirdTextures {
        up: asset_server.load("sprites/yellowbird-upflap.png"),
        mid: asset_server.load("sprites/yellowbird-midflap.png"),
        down: asset_server.load("sprites/yellowbird-downflap.png"),
    };
    let root = commands
        .spawn((GameScene, Transform::default(), Visibility::Visible))
        .id();

    commands.entity(root).with_children(|parent| {
        // Player
        parent.spawn((
            Sprite {
                image: textures.up.clone(),
                ..default()
            },
            Transform::from_xyz(-150., 70., Z_POS_PLAYER),
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
    // Move background
    for mut transform in &mut bg_query {
        transform.translation.x -= BG_SPEED;

        if transform.translation.x < -BG_IMG_DIMENSIONS.0 * 1.5 {
            transform.translation.x = BG_IMG_DIMENSIONS.0 * 1.5;
        }
    }

    // Move platform
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

        if transform.translation.y > MAX_HEIGHT {
            transform.translation.y = MAX_HEIGHT;
            **velocity = 0.0;
        }

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
    // Jump
    if keyboard.just_pressed(KeyCode::Space) {
        for mut velocity in &mut player_query {
            *velocity = Velocity(JUMP_IMPULSE)
        }
    }
}

fn setup_gameover(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        // Transform::from_xyz(0., 0., Z_POS_GAMEOVER),
        BackgroundColor(Color::srgba(0., 0., 0., 0.8)),
        GameOverLayer,
        children![
            (
                // Game over label
                Node {
                    margin: UiRect::top(Val::Px(100.0)),
                    ..default()
                },
                Text("Game Over".to_string()),
                TextFont {
                    font_size: 70.0,
                    line_height: LineHeight::RelativeToFont(2.0),
                    ..default()
                }
            ),
            (
                // Buttons
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        // Main menu
                        Node {
                            width: Val::Percent(30.),
                            height: Val::Percent(20.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        Button,
                        BackgroundColor(BUTTON_COLOR_IDLE),
                        GameOverMenuButton::MainMenu,
                        children![Text("Main Menu".to_string())],
                    ),
                    (
                        // Retry
                        Node {
                            width: Val::Percent(30.),
                            height: Val::Percent(20.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        Button,
                        BackgroundColor(BUTTON_COLOR_IDLE),
                        GameOverMenuButton::Retry,
                        children![Text("Retry".to_string())],
                    ),
                ]
            )
        ],
    ));
}

type QueryButton<'w, 's, 'a> = Query<
    'w,
    's,
    (
        &'a Interaction,
        &'a GameOverMenuButton,
        &'a mut BackgroundColor,
    ),
    (With<Button>, Changed<Interaction>),
>;

fn handle_gameover_menu_button(
    mut button_query: QueryButton,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button, mut color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(BUTTON_COLOR_PRESSED);

                match button {
                    GameOverMenuButton::MainMenu => {
                        app_state.set(AppState::MainMenu);
                    }
                    GameOverMenuButton::Retry => {
                        app_state.set(AppState::InGame);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(BUTTON_COLOR_HOVER);
            }
            Interaction::None => {
                *color = BackgroundColor(BUTTON_COLOR_IDLE);
            }
        }
    }
}

fn cleanup(
    mut commands: Commands,
    game_query: Query<Entity, With<GameScene>>,
    game_over_query: Query<Entity, With<GameOverLayer>>,
) {
    for entity in &game_query {
        commands.entity(entity).despawn();
    }

    for entity in &game_over_query {
        commands.entity(entity).despawn();
    }
}

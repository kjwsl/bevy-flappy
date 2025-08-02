use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::text::LineHeight;
use bevy_flappy_macros::hex_to_color;
use rand::Rng;

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
const Z_POS_PIPE: f32 = -4.0;
const Z_POS_PLAYER: f32 = 10.0;

const BG_SPEED: f32 = 0.2;
const PLATFORM_SPEED: f32 = 1.0;
const PIPE_SPEED: f32 = 1.0;

const GRAVITY: f32 = -700.0;
const JUMP_IMPULSE: f32 = 300.0;
const MAX_FALL_SPEED: f32 = -700.0;
const MAX_HEIGHT: f32 = GAME_DIMENSIONS.1 / 2.0;

const PIPE_WIDTH: f32 = 52.0;
const PIPE_HEIGHT: f32 = 320.0;
const PIPE_LEGROOM: f32 = 100.0;
const MAX_PIPE_GAP: f32 = 200.0;
const MIN_PIPE_GAP: f32 = 100.0;

const INITIAL_PIPE_INTERVAL: f32 = 3.0;

#[derive(Resource)]
pub struct PipeInterval(Timer);

impl PipeInterval {
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

impl Default for PipeInterval {
    fn default() -> Self {
        Self(Timer::from_seconds(
            INITIAL_PIPE_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component)]
pub enum GameOverMenuButton {
    Retry,
    MainMenu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeInterval::default())
            .insert_resource(Score::default())
            .add_systems(OnEnter(AppState::InGame), (setup, setup_ui))
            .add_systems(
                Update,
                (
                    move_bg,
                    apply_gravity,
                    handle_jump_input,
                    detect_gameover,
                    generate_pipes,
                    move_pipes,
                    destory_pipes,
                    update_score,
                )
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

#[derive(Component)]
struct GameWorld;

#[derive(Component)]
struct GameUi;

#[derive(Component)]
struct GameOverLayer;

#[derive(Component)]
struct BackgroundImage;

#[derive(Component)]
struct PlatformImage;

#[derive(Component)]
#[require(Sprite, Transform)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(pub f32);

#[derive(Component, Default)]
struct Collider;

#[derive(Resource, Clone)]
struct BirdTextures {
    up: Handle<Image>,
    mid: Handle<Image>,
    down: Handle<Image>,
}

#[derive(Resource, Clone, Default)]
struct Score(u32);

#[derive(Component, Clone)]
#[require(Text)]
struct ScoreText;

#[derive(Component, Clone)]
#[require(Sprite, Transform, Collider)]
struct Pipe;

#[derive(Component, Clone)]
struct PipePair;

#[derive(Resource, Clone)]
struct PipeTextures {
    green_pipe: Handle<Image>,
    red_pipe: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, score: ResMut<Score>) {
    let bird_textures = BirdTextures {
        up: asset_server.load("sprites/yellowbird-upflap.png"),
        mid: asset_server.load("sprites/yellowbird-midflap.png"),
        down: asset_server.load("sprites/yellowbird-downflap.png"),
    };
    let pipe_texture = PipeTextures {
        green_pipe: asset_server.load("sprites/pipe-green.png"),
        red_pipe: asset_server.load("sprites/pipe-red.png"),
    };

    let bird_image = bird_textures.up.clone();

    commands.insert_resource(bird_textures);
    commands.insert_resource(pipe_texture);

    let root = commands
        .spawn((GameWorld, Transform::default(), Visibility::Visible))
        .id();

    commands.entity(root).with_children(|parent| {
        // Player
        parent.spawn((
            Sprite {
                image: bird_image,
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
}

fn setup_ui(mut commands: Commands, mut score: ResMut<Score>) {
    score.0 = 0;
    let root = commands
        .spawn((
            GameUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        // Score
        parent.spawn((
            ScoreText,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(30.0),
                ..default()
            },
            Text(score.0.to_string()),
            TextFont {
                font_size: 50.0,
                line_height: LineHeight::RelativeToFont(2.0),
                ..default()
            },
            TextColor(Color::WHITE),
            TextShadow {
                offset: Vec2::splat(0.8),
                color: Color::BLACK,
            },
        ));
    });
}

fn detect_gameover(
    player_query: Single<&Transform, With<Player>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let transform = player_query.into_inner();

    if transform.translation.y < -BG_IMG_DIMENSIONS.1 / 2.0 - 30.0 {
        app_state.set(AppState::GameOver);
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

fn generate_pipes(
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

fn destory_pipes(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    let threshold = -GAME_DIMENSIONS.0 / 2.0 - PIPE_WIDTH / 2.0;
    for (entity, transform) in &query {
        if transform.translation.x < threshold {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_collision(collider_query: Query<(Entity, &Transform, &Sprite), With<Collider>>) {
    for (entity, transform, sprite) in &collider_query {}
    todo!()
}

fn move_pipes(mut query: Query<(&mut Transform, &PipePair)>) {
    for (mut transform, _) in &mut query {
        transform.translation.x -= PIPE_SPEED;
    }
}

fn update_score(
    player_query: Single<&Transform, With<Player>>,
    pipe_pairs_query: Query<&Transform, With<PipePair>>,
    mut score: ResMut<Score>,
    score_text_query: Single<&mut Text, With<ScoreText>>,
) {
    let player = player_query.into_inner();
    let mut score_text = score_text_query.into_inner();

    for transform in &pipe_pairs_query {
        let threshold = transform.translation.x + PIPE_WIDTH / 2.0;

        if player.translation.x == threshold {
            score.0 += 1;
            *score_text = Text(score.0.to_string());
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
    game_world_query: Single<Entity, With<GameWorld>>,
    game_ui_query: Single<Entity, With<GameUi>>,
    game_over_query: Single<Entity, With<GameOverLayer>>,
) {
    let world = game_world_query.into_inner();
    commands.entity(world).despawn();

    let ui = game_ui_query.into_inner();
    commands.entity(ui).despawn();

    let game_over = game_over_query.into_inner();
    commands.entity(game_over).despawn();
}

use bevy::prelude::*;
use crate::game::{
    constants::*,
    components::*,
    events::AudioEvent,
    collision::check_collision,
    audio::{play_audio_events, play_background_music, stop_background_music},
    player::{apply_gravity, handle_jump_input, detect_gameover},
    pipes::{generate_pipes, move_pipes, destroy_pipes},
    score::update_score,
    ui::{setup_ui, setup_gameover, handle_gameover_menu_button},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeInterval::default())
            .insert_resource(Score::default())
            .insert_resource(Difficulty::default())
            .add_event::<AudioEvent>()
            .add_systems(OnEnter(AppState::InGame), (setup, setup_ui, play_background_music))
            .add_systems(
                Update,
                (
                    move_bg,
                    apply_gravity,
                    handle_jump_input,
                    detect_collisions,
                    detect_gameover,
                    generate_pipes,
                    move_pipes,
                    destroy_pipes,
                    update_score,
                    play_audio_events,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), stop_background_music)
            .add_systems(OnEnter(AppState::GameOver), setup_gameover)
            .add_systems(
                Update,
                (handle_gameover_menu_button).run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bird_textures = BirdTextures {
        up: asset_server.load("sprites/yellowbird-upflap.png"),
        mid: asset_server.load("sprites/yellowbird-midflap.png"),
        down: asset_server.load("sprites/yellowbird-downflap.png"),
    };
    let pipe_texture = PipeTextures {
        green_pipe: asset_server.load("sprites/pipe-green.png"),
        red_pipe: asset_server.load("sprites/pipe-red.png"),
    };

    let game_sounds = GameSounds {
        wing: asset_server.load("audio/wing.ogg"),
        point: asset_server.load("audio/point.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
        die: asset_server.load("audio/die.ogg"),
    };

    let bird_image = bird_textures.up.clone();

    commands.insert_resource(bird_textures);
    commands.insert_resource(pipe_texture);
    commands.insert_resource(game_sounds);

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
            Collider, // Add collider to player
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

fn detect_collisions(
    player_query: Single<&GlobalTransform, (With<Player>, With<Collider>)>,
    pipe_query: Query<&GlobalTransform, (With<Pipe>, With<Collider>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    let player_transform = player_query.into_inner();

    for pipe_transform in &pipe_query {
        if check_collision(player_transform, pipe_transform) {
            // Send hit sound event
            audio_events.write(AudioEvent::Hit);
            app_state.set(AppState::GameOver);
            return;
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
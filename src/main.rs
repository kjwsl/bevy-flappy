use bevy::{prelude::*, window::EnabledButtons};
use bevy_flappy::{
    game::{GAME_DIMENSIONS, GamePlugin},
    main_menu::MainMenuPlugin,
    settings::SettingsPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Flappy".to_string(),
                resolution: GAME_DIMENSIONS.into(),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(SettingsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

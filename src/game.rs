use bevy::prelude::*;

use crate::main_menu::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            justify_content: JustifyContent::Center,
            ..default()
        },
        Sprite {
            image: asset_server.load("sprites/pipe-green.png"),
            ..default()
        },
    ));
}

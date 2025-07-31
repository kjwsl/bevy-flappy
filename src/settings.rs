use bevy::prelude::*;
use bevy_flappy_macros::hex_to_color;

use crate::game::AppState;

const MENU_BG_COLOR: Color = hex_to_color!("#e4ede6");
const BUTTON_COLOR_IDLE: Color = hex_to_color!("#c3d8d2");
const BUTTON_COLOR_HOVER: Color = hex_to_color!("#f4f5f4");
const BUTTON_COLOR_PRESSED: Color = hex_to_color!("#c3d8d2");

#[derive(Resource)]
pub struct Volume(pub f32);

pub struct SettingsPlugin;

#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SettingsOption {
    Back,
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Volume(0.5))
            .add_systems(OnEnter(AppState::Settings), setup)
            .add_systems(Update, (handle_input).run_if(in_state(AppState::Settings)))
            .add_systems(OnExit(AppState::Settings), cleanup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(MENU_BG_COLOR),
        SettingsMenu,
        children![(
            Node {
                width: Val::Percent(50.0),
                height: Val::Percent(30.0),
                ..default()
            },
            BackgroundColor(BUTTON_COLOR_IDLE),
            Button,
            SettingsOption::Back,
            children![Text("Back".to_string()),],
        )],
    ));
}

type QueryButton<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a Interaction, &'a SettingsOption, &'a mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
>;

fn cleanup(mut commands: Commands, query: Query<Entity, With<SettingsMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_input(mut interaction_query: QueryButton, mut app_state: ResMut<NextState<AppState>>) {
    for (interaction, button_type, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    SettingsOption::Back => {
                        println!("Back");
                        app_state.set(AppState::MainMenu);
                    }
                }
                *bg_color = BackgroundColor(BUTTON_COLOR_PRESSED);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(BUTTON_COLOR_HOVER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(BUTTON_COLOR_IDLE);
            }
        }
    }
}

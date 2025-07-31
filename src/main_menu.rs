use bevy::prelude::*;
use bevy_flappy_macros::hex_to_color;

use crate::game::AppState;

const MENU_BG_COLOR: Color = hex_to_color!("#e4ede6");
const BUTTON_COLOR_IDLE: Color = hex_to_color!("#c3d8d2");
const BUTTON_COLOR_HOVER: Color = hex_to_color!("#f4f5f4");
const BUTTON_COLOR_PRESSED: Color = hex_to_color!("#c3d8d2");

type QueryButton<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a Interaction, &'a mut BackgroundColor, &'a MenuButton),
    (Changed<Interaction>, With<Button>),
>;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MenuButton {
    Play,
    Settings,
    Quit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(Update, handle_input)
            .add_systems(OnExit(AppState::MainMenu), cleanup)
            .init_state::<AppState>();
    }
}

fn setup(mut commands: Commands) {
    fn create_button(text: String, button_type: MenuButton) -> impl Bundle {
        (
            Button,
            Node {
                width: Val::Percent(40.0),
                height: Val::Percent(15.0),
                margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
            BoxShadow(vec![ShadowStyle {
                color: Color::BLACK,
                blur_radius: Val::Px(2.0),
                x_offset: Val::Px(0.0),
                y_offset: Val::Px(0.0),
                ..default()
            }]),
            children![(
                Text(text),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
            )],
            BackgroundColor(BUTTON_COLOR_IDLE),
            button_type,
        )
    }
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        MainMenu,
        BackgroundColor(MENU_BG_COLOR),
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                margin: UiRect::bottom(Val::Px(50.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                create_button("Play".to_string(), MenuButton::Play),
                create_button("Settings".to_string(), MenuButton::Settings),
                create_button("Quit".to_string(), MenuButton::Quit),
            ]
        )],
    ));
}

fn handle_input(
    mut interaction_query: QueryButton,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(BUTTON_COLOR_PRESSED);

                match button_type {
                    MenuButton::Play => {
                        println!("Play");
                        app_state.set(AppState::InGame);
                    }
                    MenuButton::Settings => {
                        println!("Settings");
                        app_state.set(AppState::Settings);
                    }
                    MenuButton::Quit => {
                        println!("Quit");
                        exit.write(AppExit::Success);
                    }
                }
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

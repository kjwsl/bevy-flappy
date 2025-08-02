use bevy::prelude::*;
use bevy::text::LineHeight;
use crate::game::{
    constants::*,
    components::*,
};

pub fn setup_ui(mut commands: Commands, mut score: ResMut<Score>) {
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

pub fn setup_gameover(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
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

pub fn handle_gameover_menu_button(
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
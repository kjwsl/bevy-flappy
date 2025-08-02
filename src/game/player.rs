use bevy::prelude::*;
use crate::game::{
    constants::*,
    components::*,
    events::AudioEvent,
};

pub fn apply_gravity(
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

        transform.rotation =
            Quat::from_rotation_z((**velocity / MAX_FALL_SPEED.abs() * 1.5).clamp(
                -MAX_PLAYER_ROTATION.to_radians(),
                MAX_PLAYER_ROTATION.to_radians(),
            ));

        sprite.image = if **velocity > 150.0 {
            textures.up.clone()
        } else if **velocity < -150.0 {
            textures.down.clone()
        } else {
            textures.mid.clone()
        };
    }
}

pub fn handle_jump_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    // Jump
    if keyboard.just_pressed(KeyCode::Space) {
        for mut velocity in &mut player_query {
            *velocity = Velocity(JUMP_IMPULSE)
        }
        // Send wing sound event
        audio_events.write(AudioEvent::Wing);
    }
}

pub fn detect_gameover(
    player_query: Single<&Transform, With<Player>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    let transform = player_query.into_inner();

    if transform.translation.y < -BG_IMG_DIMENSIONS.1 / 2.0 - 30.0 {
        // Send die sound event
        audio_events.write(AudioEvent::Die);
        app_state.set(AppState::GameOver);
    }
} 
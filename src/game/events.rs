use bevy::prelude::*;

#[derive(Event)]
pub enum AudioEvent {
    Wing,
    Point,
    Hit,
    Die,
} 
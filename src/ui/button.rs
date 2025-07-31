use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct ButtonColors {
    pub normal: Color,
    pub hover: Color,
    pub pressed: Color,
}

#[derive(Default, Clone)]
pub struct ButtonStyle {
    pub colors: ButtonColors,
    pub font: Handle<Font>,
}


#[derive(Bundle)]
pub struct ButtonBundle {


}

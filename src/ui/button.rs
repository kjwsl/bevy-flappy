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
    pub node: Node,
    pub button: Button,
    pub background_color: BackgroundColor,
    pub interaction: Interaction,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for ButtonBundle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            button: Button,
            background_color: BackgroundColor::default(),
            interaction: Interaction::None,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

impl ButtonBundle {
    pub fn with_style(style: ButtonStyle) -> Self {
        Self {
            background_color: BackgroundColor(style.colors.normal),
            ..default()
        }
    }
}

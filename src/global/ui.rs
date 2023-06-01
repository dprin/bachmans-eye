use bevy::prelude::*;

#[derive(Component)]
pub struct Button;

pub fn button_animations() {}

mod color {
    use bevy::prelude::*;

    const NORMAL: Color = Color::hsl(30.0, 0.1, 0.1);
    const HOVER: Color = Color::hsl(30.0, 0.1, 0.2);
    const ACTIVE: Color = Color::hsl(30.0, 0.1, 0.3);
    const DISABLED: Color = Color::hsl(30.0, 0.1, 0.4);
}

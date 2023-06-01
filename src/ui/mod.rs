use bevy::prelude::*;

use crate::{global, AppState};

pub struct UIPLugin;

impl Plugin for UIPLugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(setup.in_schedule(OnEnter(AppState::Simulation)));
    }
}

fn setup(mut commands: Commands, server_asset: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(20.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: global::color::BACKGROUND.with_b(1.0).into(),
        ..default()
    });
}
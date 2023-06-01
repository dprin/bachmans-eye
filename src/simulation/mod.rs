pub mod connection;

pub mod client;
pub mod cable;

pub use client::ClientBundle;
pub use cable::CableBundle;

#[cfg(test)]
mod test;

use bevy::prelude::*;

use crate::{global, AppState};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(AppState::Simulation)))
            .add_systems((
                connection::connection_system,
                client::client_send_system,
            ).in_set(OnUpdate(AppState::Simulation)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
        size: Size::width(Val::Px(300.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },

        background_color: global::color::BACKGROUND.with_r(0.2).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Simulation",
            TextStyle {
                font: asset_server.load("fonts/AtkinsonHyperlegible-Regular.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ));
    });
}

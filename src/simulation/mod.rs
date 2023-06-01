pub mod connection;

pub mod client;
pub mod cable;

pub use client::ClientBundle;
pub use cable::CableBundle;

#[cfg(test)]
mod test;

use bevy::prelude::*;

use crate::AppState;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                connection::connection_system,
                client::client_send_system,
            ).in_set(OnUpdate(AppState::Simulation)));
    }
}

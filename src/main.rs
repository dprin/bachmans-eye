mod global;
mod menu;
mod simulation;

use bevy::{prelude::*, winit::WinitSettings};

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    Menu,
    Simulation,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(setup.on_startup())
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(global::color::BACKGROUND))
        .add_state::<AppState>()
        .add_plugin(menu::MenuPlugin)
        .add_plugin(simulation::SimulationPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

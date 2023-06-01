mod global;
mod menu;
mod simulation;
mod ui;

use bevy::{prelude::*, winit::WinitSettings};

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    Simulation,
    Settings,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(setup.on_startup())
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(global::color::BACKGROUND))
        .add_state::<AppState>()
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(simulation::SimulationPlugin)
        .add_plugin(ui::UIPLugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

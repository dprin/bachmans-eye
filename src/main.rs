mod menu;
mod global;

use bevy::{
    prelude::*,
    winit::WinitSettings,
};

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    Menu,
    PhysicalLayer,
    DataLinkLayer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_state::<AppState>()
        .add_plugin(menu::MenuPlugin)
        .run();
}


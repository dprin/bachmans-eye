// #![cfg(test)]

use super::{*, CableBundle, client::{ClientBundle, Client}, cable::Cable, connection::Sender};

#[test]
fn simple_test() {
    let mut app = App::new();

    app.add_state::<AppState>();
    app.add_plugin(SimulationPlugin);
    
    let client2 = app.world.spawn(ClientBundle {
        ..default()
    }).id();

    let cable = app.world.spawn(CableBundle {
        sender: Sender::to(client2),
        ..default()
    }).id();

    let _client1 = app.world.spawn(ClientBundle {
        sender: Sender::to(cable),
        ..default()
    }).id();

    app.update();
}

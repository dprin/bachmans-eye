use std::time::{Instant, Duration};

use bevy::prelude::*;

use super::connection::{Sender, Receiver, Payload};

#[derive(Component, Default)]
pub struct Client {
    /// Toggle that determines if it should send or not.
    pub sending: bool,
    pub config: Config,
}

#[derive(Reflect)]
pub struct Config {
    /// The rate at which the client sends data (in bits/s)
    pub send_rate: f32,

    /// The rate at which the client receives data (in bits/s)
    pub receive_rate: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            send_rate: 1.0,
            receive_rate: 1.0,
        }
    }
}

struct Inspector {
    config: Box<dyn Config>
}

impl Client {
    pub fn get_payload(&mut self, time: &Res<Time>) -> Payload {
        Payload::UniformBits {
            bits: vec![true, false, true, false, true, false, true, false],
            start_time: Instant::now(),
            end_time: Instant::now() + Duration::from_secs(1),
        }
    }
}

#[derive(Bundle, Default)]
pub struct ClientBundle {
    pub client: Client,
    pub sender: Sender,
    pub receiver: Receiver,
}

pub fn client_send_system(mut query: Query<(&mut Client, &mut Sender)>, time: Res<Time>) {
    for (mut client, mut sender) in query.iter_mut() {
        if client.sending {
            sender.payload = client.get_payload(&time);
        }
    }
}

use bevy::{prelude::*, utils::Instant};

#[derive(Default, Clone)]
pub enum Payload {
    #[default]
    Empty,
    BitsWithTimestamp(Vec<(bool, Instant)>),
    UniformBits {
        bits: Vec<bool>,
        start_time: Instant,
        end_time: Instant,
    }
}

#[derive(Default, Component)]
pub struct Sender {
    pub payload: Payload,
    pub destination: Option<Entity>,
}

impl Sender {
    pub fn to(destination: Entity) -> Self {
        Self {
            destination: Some(destination),
            ..Default::default()
        }
    }
}

#[derive(Default, Component)]
pub struct Receiver {
    recieved: Payload,
}

fn send(sender: &mut Sender, receiver: &mut Receiver) {
    receiver.recieved = sender.payload.clone();
    sender.payload = Payload::Empty;
}

/// Transfers payloads from `Sender`s to `Receiver`s. 
pub fn connection_system(
    mut sender_query: Query<&mut Sender>,
    mut receiver_query: Query<&mut Receiver>,
) {
    for mut sender in sender_query.iter_mut() {
        if let Some(destination) = sender.destination {
            let Ok(mut receiver) = receiver_query.get_mut(destination) else {
                panic!("Sender destination does not exist")
            };

            send(&mut sender, &mut receiver);
        }
    }
}

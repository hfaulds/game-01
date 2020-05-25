use amethyst::{
  derive::SystemDesc,
  ecs::{Read, System, SystemData, Write},
  network::simulation::{NetworkSimulationEvent, TransportResource},
  shrev::{EventChannel, ReaderId},
};

use std::str::from_utf8;

use log::{error, info};

#[derive(SystemDesc)]
#[system_desc(name(ServerSystemDesc))]
pub struct ServerSystem {
  #[system_desc(event_channel_reader)]
  reader_id: ReaderId<NetworkSimulationEvent>,
}

impl ServerSystem {
  fn new(reader_id: ReaderId<NetworkSimulationEvent>) -> ServerSystem {
    ServerSystem {
      reader_id: reader_id,
    }
  }
}

impl<'a> System<'a> for ServerSystem {
  type SystemData = (
    Write<'a, TransportResource>,
    Read<'a, EventChannel<NetworkSimulationEvent>>,
  );

  fn run(&mut self, (mut net, channel): Self::SystemData) {
    for event in channel.read(&mut self.reader_id) {
      match event {
        //NetworkSimulationEvent::Message(addr, payload) => {
        //info!("Received payload: {}", from_utf8(&payload[..]).unwrap());
        //net.send(*addr, b"ok");
        //}
        NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
        NetworkSimulationEvent::Disconnect(addr) => {
          info!("Client Disconnected: {}", addr);
        }
        NetworkSimulationEvent::RecvError(e) => {
          error!("Recv Error: {:?}", e);
        }
        _ => {}
      }
    }
  }
}

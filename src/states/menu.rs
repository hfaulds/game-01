use amethyst::{
  ecs::prelude::Entity,
  input::{is_close_requested, is_key_down},
  network::simulation::tcp::TcpNetworkResource,
  prelude::*,
  ui::{UiCreator, UiEvent, UiEventType, UiFinder},
  winit::VirtualKeyCode,
};

use std::net::TcpListener;

use log::{error, info};

const BUTTON_HOST: &str = "host";
const BUTTON_JOIN: &str = "join";
const INPUT_HOST: &str = "join_host";

#[derive(Default, Debug)]
pub struct Menu {
  ui_handle: Option<Entity>,

  button_host: Option<Entity>,
  button_join: Option<Entity>,
  input_host: Option<Entity>,
}

impl SimpleState for Menu {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    self.ui_handle =
      Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
  }

  fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    let StateData { world, .. } = state_data;

    if self.button_host.is_none() || self.button_join.is_none() || self.input_host.is_none() {
      world.exec(|ui_finder: UiFinder<'_>| {
        self.button_host = ui_finder.find(BUTTON_HOST);
        self.button_join = ui_finder.find(BUTTON_JOIN);
        self.input_host = ui_finder.find(INPUT_HOST);
      });
    }

    Trans::None
  }

  fn handle_event(
    &mut self,
    state_data: StateData<'_, GameData<'_, '_>>,
    event: StateEvent,
  ) -> SimpleTrans {
    let StateData { world, .. } = state_data;

    match &event {
      StateEvent::Window(event) => {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
          Trans::Quit
        } else {
          Trans::None
        }
      }
      StateEvent::Ui(UiEvent {
        event_type: UiEventType::Click,
        target: _,
      }) => {
        if let Some(_) = self.button_host {
          if let Ok(listener) = TcpListener::bind("0.0.0.0:9898") {
            world
              .fetch_mut::<TcpNetworkResource>()
              .set_listener(listener);
            let lobby = crate::states::lobby::Lobby::default();
            info!("switching to lobby");
            return Trans::Switch(Box::new(lobby));
          } else {
            error!("Error creating tcp server");
          }
        }
        Trans::None
      }

      _ => Trans::None,
    }
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    if let Some(root_entity) = self.ui_handle {
      data
        .world
        .delete_entity(root_entity)
        .expect("Failed to remove Menu");
    }

    self.ui_handle = None;
  }
}

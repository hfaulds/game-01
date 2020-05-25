use amethyst::{
  ecs::prelude::Entity,
  input::{is_close_requested, is_key_down},
  prelude::*,
  ui::UiCreator,
  winit::VirtualKeyCode,
};

use log::info;

#[derive(Debug,Default)]
pub struct Lobby {
  ui_handle: Option<Entity>,
}

impl SimpleState for Lobby {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    info!("start lobby");
    let world = data.world;

    self.ui_handle =
      Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/lobby.ron", ())));
  }

  fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
    match &event {
      StateEvent::Window(event) => {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
          Trans::Quit
        } else {
          Trans::None
        }
        // Listen for game start button click
        // Create dispatcher for simulating the game
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

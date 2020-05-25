use amethyst::{
  audio::AudioBundle,
  core::transform::TransformBundle,
  input::{InputBundle, StringBindings},
  network::simulation::tcp::TcpNetworkBundle,
  prelude::*,
  renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
  ui::{RenderUi, UiBundle},
  utils::application_root_dir,
};

extern crate log;

mod states;
mod systems;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;

  let assets_dir = app_root.join("assets");
  let config_dir = app_root.join("config");
  let display_config_path = config_dir.join("display.ron");

  let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
          RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.34, 0.36, 0.52, 1.0]),
        )
        .with_plugin(RenderUi::default()),
    )?
    .with_bundle(InputBundle::<StringBindings>::new())?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(AudioBundle::default())?
    .with_bundle(TcpNetworkBundle::new(None, 2048))?
    .with_system_desc(crate::systems::server::ServerSystemDesc::default(), "server_system", &[])
    ;

  let mut game = Application::new(assets_dir, states::menu::Menu::default(), game_data)?;
  game.run();

  Ok(())
}

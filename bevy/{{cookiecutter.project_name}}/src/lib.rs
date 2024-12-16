use bevy::{prelude::*, window::WindowResolution};

#[macro_use]
mod bevy_util;

mod camera;

pub fn plugin(app: &mut App) {
  app
    .add_plugins((
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "{{ cookiecutter.project_name | replace("-", " ") | replace("_", " ") | title }}".to_string(),
            resolution: WindowResolution::new(VIEWPORT_SIZE.x, VIEWPORT_SIZE.y),
            ..default()
          }),
          ..default()
        })
        .set(AssetPlugin {
          mode: AssetMode::Processed,
          ..default()
        })
        .set(ImagePlugin::default_nearest()),
      camera::plugin,
  ));
}

const VIEWPORT_SIZE: Vec2 = Vec2 { x: 1000.0, y: 600.0 };

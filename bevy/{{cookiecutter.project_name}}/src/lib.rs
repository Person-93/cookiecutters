use bevy::{prelude::*, window::WindowResolution};
use bevy_tweening::TweeningPlugin;

#[macro_use]
mod bevy_util;

mod camera;
mod physics;

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
      TweeningPlugin,
      camera::plugin,
      physics::plugin,
  ));
}

const VIEWPORT_SIZE: Vec2 = Vec2 { x: 1000.0, y: 600.0 };
const VIEWPORT_TOP_Y: f32 = VIEWPORT_SIZE.y / 2.0;
const VIEWPORT_BOTTOM_Y: f32 = -VIEWPORT_TOP_Y;
const VIEWPORT_RIGHT_X: f32 = VIEWPORT_SIZE.x / 2.0;
const VIEWPORT_LEFT_X: f32 = -VIEWPORT_RIGHT_X;

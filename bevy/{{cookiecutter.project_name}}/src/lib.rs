use bevy::{prelude::*, window::WindowResolution};

#[macro_use]
mod bevy_util;
use crate::numeric_conversions::ToInt as _;

mod camera;
mod numeric_conversions;

pub fn plugin(app: &mut App) {
  app.add_plugins((
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
    physics::plugin,
  ));

}

const VIEWPORT_SIZE: UVec2 = UVec2 { x: 1000, y: 600 };
const VIEWPORT_TOP_Y: f32 = VIEWPORT_SIZE.y as f32 / 2.0;
const VIEWPORT_BOTTOM_Y: f32 = -VIEWPORT_TOP_Y;
const VIEWPORT_RIGHT_X: f32 = VIEWPORT_SIZE.x as f32 / 2.0;
const VIEWPORT_LEFT_X: f32 = -VIEWPORT_RIGHT_X;

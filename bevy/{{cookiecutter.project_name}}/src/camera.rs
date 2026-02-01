use bevy::{
  prelude::*,
  window::{PrimaryWindow, WindowResized},
};

use crate::VIEWPORT_SIZE;

pub fn plugin(app: &mut App) {
  app
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, set_viewport.run_if(on_message::<WindowResized>));
}

#[derive(Component)]
#[require(Camera2d)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
  commands
    .spawn(MainCamera)
    .entry::<Camera>()
    .and_modify(|mut camera| {
      camera.clear_color = ClearColorConfig::Custom(BACKGROUND_COLOR);
    });
}

fn set_viewport(
  window: Query<&Window, With<PrimaryWindow>>,
  mut query: Query<
    (&mut Camera, &mut Projection),
    With<MainCamera>,
  >,
) -> Result {
  let window_size = window.single()?.physical_size();

  let scale = window_size.as_vec2() / VIEWPORT_SIZE.as_vec2();
  let scale = scale.x.min(scale.y);
  let physical_size = (VIEWPORT_SIZE.as_vec2() * scale).as_uvec2();

  let (mut camera, mut projection) = query.single_mut()?;

  let viewport = camera.viewport.get_or_insert_default();
  viewport.physical_size = physical_size;
  viewport.physical_position = (window_size - physical_size) / 2;

  let projection = match &mut*projection {
    Projection::Orthographic(projection) => projection,
    _ => unreachable!("camera should use orthographic projection")
  };
  projection.scale = 1.0 / scale;

  Ok(())
}

const BACKGROUND_COLOR: Color = Color::srgb(0.29, 0.31, 0.31);

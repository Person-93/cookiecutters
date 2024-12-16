use bevy::{prelude::*, window::WindowResolution};

#[macro_use]
mod bevy_util;

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
    )
    .add_systems(Startup, spawn_camera);
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    MainCamera,
    Camera2dBundle {
      transform: Transform::from_xyz(100.0, 200.0, 0.0),
      ..default()
    },
  ));
}

const VIEWPORT_SIZE: Vec2 = Vec2 { x: 1000.0, y: 600.0 };

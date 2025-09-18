use avian2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_util::sprite_size;

#[derive(Default, Clone, Copy, PhysicsLayer)]
pub enum Layers {
  #[default]
  Default,
}

impl Layers {
  pub fn all_except_this(self) -> u32 {
    Self::all_bits() & !self.to_bits()
  }
}

pub fn plugin(app: &mut App) {
  app
    .insert_resource(Gravity(Gravity::default().0 * LENGTH_UNIT))
    .add_plugins((
      PhysicsPlugins::default().with_length_unit(LENGTH_UNIT),
      PhysicsDebugPlugin::default(),
    ))
    .add_systems(Startup, toggle_physics_debug)
    .add_systems(
      Update,
      (
        toggle_physics_debug.run_if(input_just_pressed(KeyCode::Backslash)),
        scale_collider_to_sprite,
      ),
    );
}

fn toggle_physics_debug(mut gizmo_configs: ResMut<GizmoConfigStore>) {
  let config = gizmo_configs.config_mut::<PhysicsGizmos>().0;
  config.enabled = !config.enabled;
}

#[derive(Component, Default)]
pub struct ColliderFixupMarker;

/// A system that scales rapier colliders to match the size of the
/// entity's sprite.
pub fn scale_collider_to_sprite(
  images: Res<Assets<Image>>,
  mut query: Query<(&Sprite, &mut Collider), With<ColliderFixupMarker>>,
) {
  query.iter_mut().for_each(|(sprite, mut collider)| {
    let scale = sprite_size(sprite, &images);
    collider.set_scale(scale, 1024);
  });
}

const LENGTH_UNIT: f32 = 200.;

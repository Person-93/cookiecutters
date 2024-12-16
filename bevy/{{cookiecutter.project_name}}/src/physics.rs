use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::bevy_util::sprite_size;

pub fn plugin(app: &mut App) {
  app
    .add_plugins((
      RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
      RapierDebugRenderPlugin::default().disabled(),
    ))
    .add_systems(
      Update,
      (
        toggle_physics_debug.run_if(input_just_pressed(KeyCode::Backslash)),
        scale_collider_to_sprite,
      ),
    );
}

fn toggle_physics_debug(mut debug_context: ResMut<DebugRenderContext>) {
  debug_context.enabled = !debug_context.enabled;
}

#[derive(Component, Default)]
pub struct ColliderFixupMarker;

/// A system that scales rapier colliders to match the size of the
/// entity's sprite.
fn scale_collider_to_sprite(
  mut commands: Commands,
  images: Res<Assets<Image>>,
  query: Query<(Entity, &Sprite), Added<ColliderFixupMarker>>,
) {
  for (entity, sprite) in &query {
    let scale = sprite_size(sprite, &images);
    commands
      .entity(entity)
      .insert(ColliderScale::Relative(scale))
      .remove::<ColliderFixupMarker>();
  }
}

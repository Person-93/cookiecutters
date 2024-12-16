use bevy::prelude::*;

/// Gets the size of the sprite BEFORE transforms have been applied
pub fn sprite_size(sprite: &Sprite, images: &Res<Assets<Image>>) -> Vec2 {
  sprite
    .custom_size
    .or_else(|| {
      images
        .get(sprite.image.id())
        .map(|image| image.size().as_vec2())
    })
    .expect("sprite should have a size")
}

#[macro_export]
macro_rules! get_single {
  ($query:expr) => {
    match ($query).get_single() {
      Ok(data) => data,
      Err(::bevy::ecs::query::QuerySingleError::NoEntities(_)) => return,
      err @ Err(::bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
        panic!("{err:?}")
      },
    }
  };
}

#[macro_export]
macro_rules! get_single_mut {
  ($query:expr) => {
    match ($query).get_single_mut() {
      Ok(data) => data,
      Err(::bevy::ecs::query::QuerySingleError::NoEntities(_)) => return,
      err @ Err(::bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
        panic!("{err:?}")
      },
    }
  };
}

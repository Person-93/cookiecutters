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

#![allow(unused, reason = "utils that will be useful later")]

use core::fmt::Display;

use bevy::prelude::*;

/// Gets the size of the sprite BEFORE transforms have been applied
pub fn sprite_size(sprite: &Sprite, images: &Assets<Image>) -> Vec2 {
  sprite
    .custom_size
    .or_else(|| {
      images
        .get(sprite.image.id())
        .map(|image| image.size().as_vec2())
    })
    .expect("sprite should have a size")
}

pub trait UpdateMutExt<T> {
  /// Helper function to avoid triggering bevy's change detection needlesly.
  ///
  /// The predicate takes an immutable reference and returns a `bool`. If it
  /// returns `true`, the `update` function is called with a mutable reference.
  fn update_if<Ret>(
    &mut self,
    predicate: impl FnOnce(&T) -> bool,
    update: impl FnOnce(&mut T) -> Ret,
  ) -> Option<Ret>;
}

impl<T> UpdateMutExt<T> for Mut<'_, T> {
  fn update_if<Ret>(
    &mut self,
    predicate: impl FnOnce(&T) -> bool,
    update: impl FnOnce(&mut T) -> Ret,
  ) -> Option<Ret> {
    predicate(self).then(|| update(self))
  }
}

pub trait UnwrapOrLog<T>: Sized {
  /// Unwraps the contained value or logs the error and returns the default
  #[inline]
  fn unwrap_or_log(self, default: T) -> T {
    self.unwrap_or_log_with(|| default)
  }

  /// Unwraps the contained value or logs the error and returns `Default::default`
  #[inline]
  fn unwrap_or_log_default(self) -> T
  where
    T: Default,
  {
    self.unwrap_or_log_with(T::default)
  }

  /// Unwraps the contained value or logs the error and returns the value from
  /// the provided function
  fn unwrap_or_log_with<F: FnOnce() -> T>(self, default: F) -> T;
}

impl<T, E: Display> UnwrapOrLog<T> for Result<T, E> {
  #[inline]
  fn unwrap_or_log_with<F: FnOnce() -> T>(self, default: F) -> T {
    self.unwrap_or_else(|err| {
      error!("{err}");
      default()
    })
  }
}

impl<T> UnwrapOrLog<T> for Option<T> {
  #[inline]
  fn unwrap_or_log_with<F: FnOnce() -> T>(self, default: F) -> T {
    self.unwrap_or_else(|| {
      error!("tried unwrapping an Option containing None");
      default()
    })
  }
}

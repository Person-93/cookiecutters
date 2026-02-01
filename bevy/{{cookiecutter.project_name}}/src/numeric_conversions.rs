#![allow(
  clippy::as_conversions,
  clippy::float_cmp,
  reason = "all conversions in this mod are checked before or after converting"
)]
#![allow(unused, reason = "These traits will be useful later")]

pub trait AsUsize {
  /// lossless conversion to usize
  #[expect(
    clippy::wrong_self_convention,
    reason = "named after `as` conversion"
  )]
  fn as_usize(self) -> usize;
}

macro_rules! impl_as_usize {
  ($($type:ty)*) => {
    $(
      impl AsUsize for $type {
        #[inline(always)]
        fn as_usize(self) -> usize {
          const _: () = {
            assert!(
              size_of::<$type>() <= size_of::<usize>(),
              "type is too large to convert to usize",
            );
          };
          self as usize
        }
      }
    )*
  };
}

impl_as_usize!(u8 u16 u32 usize);

pub trait AsF32 {
  /// Converts an int to f32. Panics if the int cannot be converted precisely.
  #[expect(
    clippy::wrong_self_convention,
    reason = "named after `as` conversion"
  )]
  fn as_f32(self) -> f32;
}

pub trait AsF64 {
  /// Converts an int to f64. Panics if the int cannot be converted precisely.
  #[expect(
    clippy::wrong_self_convention,
    reason = "named after `as` conversion"
  )]
  fn as_f64(self) -> f64;
}

impl AsF32 for u8 {
  #[inline(always)]
  fn as_f32(self) -> f32 {
    self.into()
  }
}

impl AsF32 for u16 {
  #[inline(always)]
  fn as_f32(self) -> f32 {
    self.into()
  }
}

const TO_F32_ERR: &str = "integer is too large to convert to f32 precisely";
const TO_F64_ERR: &str = "integer is too large to convert to f64 precisely";

impl AsF32 for u32 {
  #[inline(always)]
  fn as_f32(self) -> f32 {
    assert!(self < 2u32.pow(25) - 1, "{TO_F32_ERR}");
    self as f32
  }
}

impl AsF32 for u64 {
  #[inline(always)]
  fn as_f32(self) -> f32 {
    assert!(self < 2u64.pow(25) - 1, "{TO_F32_ERR}");
    self as f32
  }
}

impl AsF32 for usize {
  #[inline(always)]
  fn as_f32(self) -> f32 {
    assert!(self < 2usize.pow(25) - 1, "{TO_F32_ERR}");
    self as f32
  }
}

impl AsF32 for i32 {
  #[inline(always)]
  fn as_f32(self) -> f32 {
    assert!(
      self < 2i32.pow(25) - 1 && self > -(2i32.pow(24)) + 1,
      "{TO_F32_ERR}"
    );
    self as f32
  }
}

impl AsF64 for u64 {
  #[inline(always)]
  fn as_f64(self) -> f64 {
    assert!(self < 2u64.pow(53), "{TO_F64_ERR}");
    self as f64
  }
}

impl AsF64 for usize {
  #[inline(always)]
  fn as_f64(self) -> f64 {
    #[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32")))]
    assert!(self < 2usize.pow(53), "{TO_F64_ERR}");
    self as f64
  }
}

pub trait ToInt<T> {
  /// Converts the float to an int. Panics if the float cannot be converted
  /// precisely.
  fn cast_int(self) -> T;

  /// Truncates the float and converts to an int. Panics if the truncated
  /// float cannot be converted precisely.
  fn trunc_int(self) -> T;
}

macro_rules! impl_to_int {
  ($float:ty => $int:ty) => {
    impl ToInt<$int> for $float {
      #[inline(always)]
      fn cast_int(self) -> $int {
        let int = self as _;
        assert!(self == int as _);
        int
      }

      #[inline(always)]
      fn trunc_int(self) -> $int {
        self.trunc().cast_int()
      }
    }
  };
}

impl_to_int!(f32 => i32);
impl_to_int!(f32 => u32);
impl_to_int!(f32 => usize);

#[inline(never)]
#[cold]
pub fn panic_index_out_of_bounds() -> ! {
  panic!()
}

#[inline(always)]
pub fn encode_bool(x: bool) -> [u8; 1] {
  [x as u8]
}

#[inline(always)]
pub fn encode_f32(x: f32) -> [u8; 4] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_f64(x: f64) -> [u8; 8] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_i8(x: i8) -> [u8; 1] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_i16(x: i16) -> [u8; 2] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_i32(x: i32) -> [u8; 4] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_i64(x: i64) -> [u8; 8] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_i128(x: i128) -> [u8; 16] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_u8(x: u8) -> [u8; 1] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_u16(x: u16) -> [u8; 2] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_u32(x: u32) -> [u8; 4] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_u64(x: u64) -> [u8; 8] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn encode_u128(x: u128) -> [u8; 16] {
  x.to_le_bytes()
}

#[inline(always)]
pub fn try_decode_bool(x: [u8; 1]) -> Option<bool> {
  match x[0] {
    0 => Some(false),
    1 => Some(true),
    _ => None,
  }
}

#[inline(always)]
pub fn decode_f32(x: [u8; 4]) -> f32 {
  f32::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_f64(x: [u8; 8]) -> f64 {
  f64::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_i8(x: [u8; 1]) -> i8 {
  i8::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_i16(x: [u8; 2]) -> i16 {
  i16::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_i32(x: [u8; 4]) -> i32 {
  i32::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_i64(x: [u8; 8]) -> i64 {
  i64::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_i128(x: [u8; 16]) -> i128 {
  i128::from_le_bytes(x)
}
#[inline(always)]
pub fn decode_u8(x: [u8; 1]) -> u8 {
  u8::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_u16(x: [u8; 2]) -> u16 {
  u16::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_u32(x: [u8; 4]) -> u32 {
  u32::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_u64(x: [u8; 8]) -> u64 {
  u64::from_le_bytes(x)
}

#[inline(always)]
pub fn decode_u128(x: [u8; 16]) -> u128 {
  u128::from_le_bytes(x)
}

pub mod unchecked {
  #[inline(always)]
  pub unsafe fn get_array<const N: usize>(x: &[u8], i: usize) -> &[u8; N] {
    let p = x.as_ptr();
    let p = unsafe { p.add(i) };
    let p = p as *const [u8; N];
    unsafe { &*p }
  }

  #[inline(always)]
  pub unsafe fn get_array_mut<const N: usize>(x: &mut [u8], i: usize) -> &mut [u8; N] {
    let p = x.as_ptr();
    let p = unsafe { p.add(i) };
    let p = p as *mut [u8; N];
    unsafe { &mut *p }
  }

  #[inline(always)]
  pub unsafe fn get_slice(x: &[u8], i: usize, j: usize) -> &[u8] {
    unsafe { x.get_unchecked(i .. j) }
  }

  #[inline(always)]
  pub unsafe fn decode_bool(x: [u8; 1]) -> bool {
    match x[0] {
      0 => false,
      1 => true,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }
}

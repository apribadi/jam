#![no_std]

/// A `Value` is ...???
///
/// SAFETY:
///
/// A sound implementation of the `Value` trait must satisfy the following
/// properties.
///
/// - ???

pub unsafe trait Value: Copy {
  const SIZE: u32;

  /// SAFETY:
  ///
  /// ???

  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self);
}

/// An `Object` is ...???
///
/// SAFETY:
///
/// ???

pub unsafe trait Object {
  /// SAFETY:
  ///
  /// ???

  unsafe fn new(slice: &[u8]) -> &Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn new_mut(slice: &mut [u8]) -> &mut Self;
}

/// A `SizedObject` is ...???
///
/// SAFETY:
///
/// ???

pub unsafe trait SizedObject: Object {
  const SIZE: u32;
}

/// An array of values.
///
/// Supports O(1) bounds-checked access by index.

#[repr(transparent)]
pub struct ArrayV<T>
where
  T: Value
{
  _marker: core::marker::PhantomData<fn(T) -> T>,
  data: [u8],
}

unsafe impl<T> Object for ArrayV<T>
where
  T: Value
{
  #[inline(always)]
  unsafe fn new(slice: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(slice) }
  }

  #[inline(always)]
  unsafe fn new_mut(slice: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute::<&mut [u8], &mut Self>(slice) }
  }
}

impl<T> ArrayV<T>
where
  T: Value
{
  const STRIDE: u32 = max(T::SIZE, 1);

  /// ???

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.data.len() as u32 / Self::STRIDE
  }

  /// ???
  ///
  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: u32) -> T {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let i = i as u32;

    unsafe { T::read(&self.data, &mut {i}) }
  }

  /// ???
  ///
  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn set(&mut self, index: u32, value: T) {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let i = i as u32;

    unsafe { T::write(&mut self.data, &mut {i}, value) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = T> {
    IterArrayV(self)
  }

  #[inline(never)]
  #[cold]
  fn panic_out_of_bounds() -> ! {
    panic!()
  }

  #[inline(always)]
  unsafe fn get_unchecked(&self, index: u32) -> T {
    let i = Self::STRIDE * index;
    unsafe { T::read(&self.data, &mut {i}) }
  }

  #[inline(always)]
  unsafe fn pop_unchecked(&self) -> &Self {
    unsafe { Self::new(self.data.get_unchecked(Self::STRIDE as usize ..)) }
  }
}

struct IterArrayV<'a, T>(&'a ArrayV<T>)
where
  T: Value;

impl<'a, T> Iterator for IterArrayV<'a, T>
where
  T: Value
{
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() {
      None
    } else {
      let x = unsafe { self.0.get_unchecked(0) };
      self.0 = unsafe { self.0.pop_unchecked() };
      Some(x)
    }
  }
}

/// An array of sized objects.
///
/// Supports O(1) bounds-checked access by index.

#[repr(transparent)]
pub struct ArrayO<T>
where
  T: ?Sized + SizedObject
{
  _marker: core::marker::PhantomData<fn(T) -> T>,
  data: [u8],
}

unsafe impl<T> Object for ArrayO<T>
where
  T: ?Sized + SizedObject
{
  #[inline(always)]
  unsafe fn new(slice: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(slice) }
  }

  #[inline(always)]
  unsafe fn new_mut(slice: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute::<&mut [u8], &mut Self>(slice) }
  }
}

impl<T> ArrayO<T>
where
  T: ?Sized + SizedObject
{
  const STRIDE: u32 = max(T::SIZE, 1);

  /// ???

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.data.len() as u32 / Self::STRIDE
  }

  /// ???
  ///
  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: u32) -> &T {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let i = i as u32;
    let j = i + Self::STRIDE;

    unsafe { T::new(subslice(&self.data, i, j)) }
  }

  /// ???
  ///
  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get_mut(&mut self, index: u32) -> &mut T {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let i = i as u32;
    let j = i + Self::STRIDE;

    unsafe { T::new_mut(subslice_mut(&mut self.data, i, j)) }
  }

  /*

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = &'_ T> {
    IterArrayO(self)
  }

  */

  #[inline(never)]
  #[cold]
  fn panic_out_of_bounds() -> ! {
    panic!()
  }
}

unsafe impl Value for u8 {
  const SIZE: u32 = 1;

  #[inline(always)]
  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(slice, offset) })
  }

  #[inline(always)]
  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(slice, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for u32 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(slice, offset) })
  }

  #[inline(always)]
  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(slice, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for u64 {
  const SIZE: u32 = 8;

  #[inline(always)]
  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(slice, offset) })
  }

  #[inline(always)]
  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(slice, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for bool {
  const SIZE: u32 = 1;

  #[inline(always)]
  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self {
    0 != unsafe { u8::read(slice, offset) }
  }

  #[inline(always)]
  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { u8::write(slice, offset, value as u8) }
  }
}

unsafe impl<T> Value for Option<T>
where
  T: Value
{
  const SIZE: u32 = 1 + T::SIZE;

  #[inline(always)]
  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self {
    match unsafe { u8::read(slice, offset) } {
      0 => None,
      _ => Some(unsafe { T::read(slice, offset) })
    }
  }

  #[inline(always)]
  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self) {
    match value {
      None => {
        unsafe { u8::write(slice, offset, 0) };
      }
      Some(x) => {
        unsafe { u8::write(slice, offset, 1) };
        unsafe { T::write(slice, offset, x) };
      }
    }
  }
}

unsafe impl<T, E> Value for Result<T, E>
where
  T: Value,
  E: Value
{
  const SIZE: u32 = 1 + max(T::SIZE, E::SIZE);

  #[inline(always)]
  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self {
    match unsafe { u8::read(slice, offset) } {
      0 => Ok(unsafe { T::read(slice, offset) }),
      _ => Err(unsafe { E::read(slice, offset) })
    }
  }

  #[inline(always)]
  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self) {
    match value {
      Ok(x) => {
        unsafe { u8::write(slice, offset, 0) };
        unsafe { T::write(slice, offset, x) };
      }
      Err(e) => {
        unsafe { u8::write(slice, offset, 1) };
        unsafe { E::write(slice, offset, e) };
      }
    }
  }
}

#[inline(always)]
const fn max(x: u32, y: u32) -> u32 {
  if x >= y { x } else { y }
}

#[inline(always)]
unsafe fn subslice(slice: &[u8], start: u32, stop: u32) -> &[u8] {
  unsafe { slice.get_unchecked(start as usize .. stop as usize) }
}

#[inline(always)]
unsafe fn subslice_mut(slice: &mut [u8], start: u32, stop: u32) -> &mut [u8] {
  unsafe { slice.get_unchecked_mut(start as usize .. stop as usize) }
}

#[inline(always)]
unsafe fn subchunk<const N: usize>(slice: &[u8], offset: u32) -> &[u8; N] {
  let p = slice.as_ptr();
  let p = unsafe { p.add(offset as usize) };
  let p = p as *const [u8; N];
  unsafe { &*p }
}

#[inline(always)]
unsafe fn subchunk_mut<const N: usize>(slice: &mut [u8], offset: u32) -> &mut [u8; N] {
  let p = slice.as_mut_ptr();
  let p = unsafe { p.add(offset as usize) };
  let p = p as *mut [u8; N];
  unsafe { &mut *p }
}

#[inline(always)]
unsafe fn read_chunk<const N: usize>(slice: &[u8], offset: &mut u32) -> [u8; N] {
  let p = unsafe { subchunk(slice, *offset) };
  *offset += N as u32;
  *p
}

#[inline(always)]
unsafe fn write_chunk<const N: usize>(slice: &mut [u8], offset: &mut u32, value: [u8; N]) {
  let p = unsafe { subchunk_mut(slice, *offset) };
  *offset += N as u32;
  *p = value;
}

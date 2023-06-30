#![no_std]

pub mod rt;

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
  const STRIDE: u32 = if T::SIZE == 0 { 1 } else { T::SIZE };

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
  /// ???

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.data.len() as u32 / T::SIZE
  }

  /// ???
  ///
  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: u32) -> &T {
    let i = T::SIZE as u64 * index as u64;
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let i = i as usize;
    let j = i + T::SIZE as usize;

    unsafe { T::new(self.data.get_unchecked(i .. j)) }
  }

  /// ???
  ///
  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get_mut(&mut self, index: u32) -> &mut T {
    let i = T::SIZE as u64 * index as u64;
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let i = i as usize;
    let j = i + T::SIZE as usize;

    unsafe { T::new_mut(self.data.get_unchecked_mut(i .. j)) }
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

unsafe impl Value for u32 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn read(slice: &[u8], offset: &mut u32) -> Self {
    u32::from_le_bytes(unsafe { rt::read_chunk(slice, offset) })
  }

  unsafe fn write(slice: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { rt::write_chunk(slice, offset, value.to_le_bytes()) }
  }
}

/*

unsafe impl Value for u64 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn read(ptr: *const u8) -> u64 {
    unsafe { core::ptr::read_unaligned(ptr as *const u64) }
  }

  #[inline(always)]
  unsafe fn write(ptr: *mut u8, value: u64) {
    unsafe { core::ptr::write_unaligned(ptr as *mut u64, value) }
  }
}
*/

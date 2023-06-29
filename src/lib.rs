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
  /// The memory at `[ptr, ptr + SIZE)` must be valid for reading and
  /// initialized. It is not required for `ptr` to be aligned.

  unsafe fn is_valid(ptr: *const u8) -> bool;

  /// SAFETY:
  ///
  /// The memory at `[ptr, ptr + SIZE)` must be valid for reading and
  /// initialized. It is not required for `ptr` to be aligned.
  ///
  /// The contents of the memory must have been checked by `is_valid` or must
  /// have been written by a previous call to `write`.

  unsafe fn read(ptr: *const u8) -> Self;

  /// SAFETY:
  ///
  /// The memory at `[ptr, ptr + SIZE)` must be valid for writing. It is not
  /// required for `ptr` to be initialized or aligned.

  unsafe fn write(ptr: *mut u8, value: Self);
}

/// An `Object` is ...???
///
/// SAFETY:
///
/// ???

pub unsafe trait UnsizedObject {
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

pub unsafe trait Object {
  /// ???

  const SIZE: u32;

  /// SAFETY:
  ///
  /// ???

  unsafe fn new(slice: &[u8]) -> &Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn new_mut(slice: &mut [u8]) -> &mut Self;
}

/// An array of values.
///
/// Supports O(1) bounds-checked access by index.

#[repr(transparent)]
pub struct ArrayV<T: Value> {
  _marker: core::marker::PhantomData<fn(T) -> T>,
  data: [u8],
}

/// SAFETY:
///
/// - ???
/// - T::SIZE > 0 ???

unsafe impl<T: Value> UnsizedObject for ArrayV<T> {
  #[inline(always)]
  unsafe fn new(slice: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(slice) }
  }

  #[inline(always)]
  unsafe fn new_mut(slice: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute::<&mut [u8], &mut Self>(slice) }
  }
}

impl<T: Value> ArrayV<T> {
  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.data.len() as u32 / T::SIZE
  }

  #[inline(never)]
  #[cold]
  fn panic_out_of_bounds() -> ! {
    panic!()
  }

  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: u32) -> T {
    let i = (T::SIZE as u64) * (index as u64);
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let p = self.data.as_ptr();
    let i = i as usize;

    unsafe { T::read(p.add(i)) }
  }

  /// Panics:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn set(&mut self, index: u32, value: T) {
    let i = T::SIZE as u64 * index as u64;
    let n = self.data.len() as u64;

    if ! (i < n) { Self::panic_out_of_bounds() }

    let p = self.data.as_mut_ptr();
    let i = i as usize;

    unsafe { T::write(p.add(i), value) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = T> {
    IterArrayV(self)
  }
}

struct IterArrayV<'a, T: Value>(&'a ArrayV<T>);

impl<'a, T: Value> Iterator for IterArrayV<'a, T> {
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.data.is_empty() {
      None
    } else {
      let p = self.0.data.as_ptr();
      let v = unsafe { T::read(p) };
      let a = unsafe { ArrayV::new(self.0.data.get_unchecked(T::SIZE as usize ..)) };
      self.0 = a;
      Some(v)
    }
  }
}

/// An array of sized objects.
///
/// Supports O(1) bounds-checked access by index.

#[repr(transparent)]
pub struct ArrayO<T: ?Sized + Object> {
  _marker: core::marker::PhantomData<fn(T) -> T>,
  data: [u8],
}

unsafe impl<T: ?Sized + Object> UnsizedObject for ArrayO<T> {
  /// SAFETY:
  ///
  /// - ???
  /// - T::SIZE > 0 ???

  #[inline(always)]
  unsafe fn new(slice: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(slice) }
  }

  #[inline(always)]
  unsafe fn new_mut(slice: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute::<&mut [u8], &mut Self>(slice) }
  }
}

impl<T: ?Sized + Object> ArrayO<T> {
  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.data.len() as u32 / T::SIZE
  }

  #[inline(never)]
  #[cold]
  fn panic_out_of_bounds() -> ! {
    panic!()
  }

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
  pub fn iter(&self) -> impl '_ + Iterator<Item = T> {
    IterArrayV(self)
  }
  */
}

unsafe impl Value for u32 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn is_valid(_: *const u8) -> bool {
    true
  }

  #[inline(always)]
  unsafe fn read(ptr: *const u8) -> u32 {
    unsafe { core::ptr::read_unaligned(ptr as *const u32) }
  }

  #[inline(always)]
  unsafe fn write(ptr: *mut u8, value: u32) {
    unsafe { core::ptr::write_unaligned(ptr as *mut u32, value) }
  }
}

unsafe impl Value for u64 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn is_valid(_: *const u8) -> bool {
    true
  }

  #[inline(always)]
  unsafe fn read(ptr: *const u8) -> u64 {
    unsafe { core::ptr::read_unaligned(ptr as *const u64) }
  }

  #[inline(always)]
  unsafe fn write(ptr: *mut u8, value: u64) {
    unsafe { core::ptr::write_unaligned(ptr as *mut u64, value) }
  }
}

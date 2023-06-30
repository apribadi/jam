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
  const SIZE: usize;

  /// SAFETY:
  ///
  /// ???

  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self);
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

  unsafe fn new(buf: &[u8]) -> &Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn new_mut(buf: &mut [u8]) -> &mut Self;
}

/// A `SizedObject` is ...???
///
/// SAFETY:
///
/// ???

pub unsafe trait SizedObject: Object {
  const SIZE: usize;
}

/// An array of values.
///
/// Supports O(1) bounds-checked access by index.

#[repr(transparent)]
pub struct ArrayV<T>
where
  T: Value
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: [u8],
}

unsafe impl<T> Object for ArrayV<T>
where
  T: Value
{
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute(buf) }
  }

  #[inline(always)]
  unsafe fn new_mut(buf: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute(buf) }
  }
}

impl<T> ArrayV<T>
where
  T: Value
{
  const STRIDE: usize = max(T::SIZE, 1);

  /// ???

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.buf.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> usize {
    self.buf.len() / Self::STRIDE
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: usize) -> T {
    if index >= self.len() { panic_out_of_bounds() }
    unsafe { self.get_unchecked(index) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: usize) -> T {
    let i = Self::STRIDE * index;
    unsafe { T::get(&self.buf, &mut {i}) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn set(&mut self, index: usize, value: T) {
    if index >= self.len() { panic_out_of_bounds() }
    unsafe { self.set_unchecked(index, value) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
    let i = Self::STRIDE * index;
    unsafe { T::set(&mut self.buf, &mut {i}, value) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = T> {
    IterArrayV {
      _pd: core::marker::PhantomData,
      buf: &self.buf,
    }
  }
}

struct IterArrayV<'a, T>
where
  T: Value
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: &'a [u8],
}

impl<'a, T> Iterator for IterArrayV<'a, T>
where
  T: Value
{
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.buf.is_empty() {
      None
    } else {
      let p = unsafe { rt::pop_slice(&mut self.buf, ArrayV::<T>::STRIDE) };
      let x = unsafe { T::get(p, &mut {0}) };
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
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: [u8],
}

unsafe impl<T> Object for ArrayO<T>
where
  T: ?Sized + SizedObject
{
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute(buf) }
  }

  #[inline(always)]
  unsafe fn new_mut(buf: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute(buf) }
  }
}

impl<T> ArrayO<T>
where
  T: ?Sized + SizedObject
{
  const STRIDE: usize = max(T::SIZE, 1);

  /// ???

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.buf.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> usize {
    self.buf.len() / Self::STRIDE
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: usize) -> &T {
    if index >= self.len() { panic_out_of_bounds() }
    let i = Self::STRIDE * index;
    let j = i + Self::STRIDE;
    unsafe { T::new(rt::get_slice(&self.buf, i, j)) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: usize) -> &T {
    let i = Self::STRIDE * index;
    let j = i + Self::STRIDE;
    unsafe { T::new(rt::get_slice(&self.buf, i, j)) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get_mut(&mut self, index: usize) -> &mut T {
    if index >= self.len() { panic_out_of_bounds() }
    unsafe { self.get_unchecked_mut(index) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
    let i = Self::STRIDE * index;
    let j = i + Self::STRIDE;
    unsafe { T::new_mut(rt::get_slice_mut(&mut self.buf, i, j)) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = &'_ T> {
    IterArrayO {
      _pd: core::marker::PhantomData,
      buf: &self.buf,
    }
  }

  /// ???

  #[inline(always)]
  pub fn iter_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut T> {
    IterMutArrayO {
      _pd: core::marker::PhantomData,
      buf: &mut self.buf,
    }
  }
}

struct IterArrayO<'a, T>
where
  T: 'a + ?Sized + SizedObject
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: &'a [u8],
}

impl<'a, T> Iterator for IterArrayO<'a, T>
where
  T: 'a + ?Sized + SizedObject
{
  type Item = &'a T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.buf.is_empty() {
      None
    } else {
      let p = unsafe { rt::pop_slice(&mut self.buf, ArrayO::<T>::STRIDE) };
      let x = unsafe { T::new(p) };
      Some(x)
    }
  }
}

struct IterMutArrayO<'a, T>
where
  T: 'a + ?Sized + SizedObject
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: &'a mut [u8],
}

impl<'a, T> Iterator for IterMutArrayO<'a, T>
where
  T: 'a + ?Sized + SizedObject
{
  type Item = &'a mut T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.buf.is_empty() {
      None
    } else {
      let p = unsafe { rt::pop_slice_mut(&mut self.buf, ArrayO::<T>::STRIDE) };
      let x = unsafe { T::new_mut(p) };
      Some(x)
    }
  }
}

unsafe impl Value for () {
  const SIZE: usize = 0;

  #[inline(always)]
  unsafe fn get(_: &[u8], _: &mut usize) -> Self {
  }

  #[inline(always)]
  unsafe fn set(_: &mut [u8], _: &mut usize, _: Self) {
  }
}

unsafe impl Value for u8 {
  const SIZE: usize = 1;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { rt::get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { rt::set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for u16 {
  const SIZE: usize = 2;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { rt::get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { rt::set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for u32 {
  const SIZE: usize = 4;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { rt::get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { rt::set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for u64 {
  const SIZE: usize = 8;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { rt::get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { rt::set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for f32 {
  const SIZE: usize = 4;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { rt::get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { rt::set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for f64 {
  const SIZE: usize = 8;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { rt::get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { rt::set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for bool {
  const SIZE: usize = 1;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    match unsafe { u8::get(buf, ofs) } {
      0 => false,
      _ => true
    }
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { u8::set(buf, ofs, value as u8) }
  }
}

unsafe impl<T> Value for Option<T>
where
  T: Value
{
  const SIZE: usize = 1 + T::SIZE;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    match unsafe { u8::get(buf, ofs) } {
      0 => None,
      _ => Some(unsafe { T::get(buf, ofs) })
    }
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    match value {
      None => {
        unsafe { u8::set(buf, ofs, 0) };
      }
      Some(x) => {
        unsafe { u8::set(buf, ofs, 1) };
        unsafe { T::set(buf, ofs, x) };
      }
    }
  }
}

unsafe impl<T, E> Value for Result<T, E>
where
  T: Value,
  E: Value
{
  const SIZE: usize = 1 + max(T::SIZE, E::SIZE);

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    match unsafe { u8::get(buf, ofs) } {
      0 => Ok(unsafe { T::get(buf, ofs) }),
      _ => Err(unsafe { E::get(buf, ofs) })
    }
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    match value {
      Ok(x) => {
        unsafe { u8::set(buf, ofs, 0) };
        unsafe { T::set(buf, ofs, x) };
      }
      Err(e) => {
        unsafe { u8::set(buf, ofs, 1) };
        unsafe { E::set(buf, ofs, e) };
      }
    }
  }
}

unsafe impl<T, const N: usize> Value for [T; N]
where
  T: Value
{
  const SIZE: usize = T::SIZE * N;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    core::array::from_fn(|_| unsafe { T::get(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    for &item in value.iter() {
      unsafe { T::set(buf, ofs, item) }
    }
  }
}

#[inline(never)]
#[cold]
fn panic_out_of_bounds() -> ! {
  panic!()
}

#[inline(always)]
const fn max(x: usize, y: usize) -> usize {
  if x >= y { x } else { y }
}

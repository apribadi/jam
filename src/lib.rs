#![no_std]

pub mod internal;

use crate::internal::get_bytes;
use crate::internal::get_slice;
use crate::internal::pop_slice;
use crate::internal::set_bytes;

pub unsafe trait Layout {
  const SIZE: usize;
}

/// A `Value` is ...???
///
/// SAFETY:
///
/// A sound implementation of the `Value` trait must satisfy the following
/// properties.
///
/// - ???

pub unsafe trait Value: Copy + Layout {
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
}

/// An array of `N` values.

#[repr(transparent)]
pub struct ArrayVN<T, const N: usize>
where
  T: Value
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: [u8],
}

unsafe impl<T, const N: usize> Object for ArrayVN<T, N>
where
  T: Value
{
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute(buf) }
  }
}

impl<T, const N: usize> ArrayVN<T, N>
where
  T: Value
{
  const STRIDE: usize = T::SIZE;

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: usize) -> T {
    let i = Self::STRIDE * index;
    unsafe { T::get(&self.buf, &mut {i}) }
  }

  #[inline(always)]
  pub fn get(&self, index: usize) -> T {
    if index >= N { panic_out_of_bounds() }
    unsafe { self.get_unchecked(index) }
  }

  #[inline(always)]
  pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
    let i = Self::STRIDE * index;
    unsafe { T::set(&mut self.buf, &mut {i}, value) }
  }

  #[inline(always)]
  pub fn set(&mut self, index: usize, value: T) {
    if index >= N { panic_out_of_bounds() }
    unsafe { self.set_unchecked(index, value) }
  }

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = T> {
    IterArrayVN { array: self, index: 0 }
  }
}

struct IterArrayVN<'a, T, const N: usize>
where
  T: Value
{
  array: &'a ArrayVN<T, N>,
  index: usize,
}

impl<'a, T, const N: usize> Iterator for IterArrayVN<'a, T, N>
where
  T: Value
{
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.index == N {
      None
    } else {
      let x = unsafe { self.array.get_unchecked(self.index) };
      self.index += 1;
      Some(x)
    }
  }
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
  pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
    let i = Self::STRIDE * index;
    unsafe { T::set(&mut self.buf, &mut {i}, value) }
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
      let p = unsafe { pop_slice(&mut self.buf, ArrayV::<T>::STRIDE) };
      let x = unsafe { T::get(p, &mut {0}) };
      Some(x)
    }
  }
}

#[repr(transparent)]
pub struct ArrayON<T, const N: usize>
where
  T: ?Sized + Layout + Object
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: [u8],
}

unsafe impl<T, const N: usize> Object for ArrayON<T, N>
where
  T: ?Sized + Layout + Object
{
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute(buf) }
  }
}

impl<T, const N: usize> ArrayON<T, N>
where
  T: ?Sized + Layout + Object
{
  const STRIDE: usize = T::SIZE;

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: usize) -> &T {
    let i = Self::STRIDE * index;
    let j = i + Self::STRIDE;
    unsafe { T::new(get_slice(&self.buf, i, j)) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: usize) -> &T {
    if index >= N { panic_out_of_bounds() }
    unsafe { self.get_unchecked(index) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = &'_ T> {
    IterArrayON { array: self, index: 0 }
  }
}

struct IterArrayON<'a, T, const N: usize>
where
  T: 'a + ?Sized + Layout + Object
{
  array: &'a ArrayON<T, N>,
  index: usize,
}

impl<'a, T, const N: usize> Iterator for IterArrayON<'a, T, N>
where
  T: 'a + ?Sized + Layout + Object
{
  type Item = &'a T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.index == N {
      None
    } else {
      let x = unsafe { self.array.get_unchecked(self.index) };
      self.index += 1;
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
  T: ?Sized + Layout + Object
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: [u8],
}

unsafe impl<T> Object for ArrayO<T>
where
  T: ?Sized + Layout + Object
{
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute(buf) }
  }
}

impl<T> ArrayO<T>
where
  T: ?Sized + Layout + Object
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
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: usize) -> &T {
    let i = Self::STRIDE * index;
    let j = i + Self::STRIDE;
    unsafe { T::new(get_slice(&self.buf, i, j)) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: usize) -> &T {
    if index >= self.len() { panic_out_of_bounds() }
    unsafe { self.get_unchecked(index) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = &'_ T> {
    IterArrayO {
      _pd: core::marker::PhantomData,
      buf: &self.buf,
    }
  }
}

struct IterArrayO<'a, T>
where
  T: 'a + ?Sized + Layout + Object
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: &'a [u8],
}

impl<'a, T> Iterator for IterArrayO<'a, T>
where
  T: 'a + ?Sized + Layout + Object
{
  type Item = &'a T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.buf.is_empty() {
      None
    } else {
      let p = unsafe { pop_slice(&mut self.buf, ArrayO::<T>::STRIDE) };
      let x = unsafe { T::new(p) };
      Some(x)
    }
  }
}

unsafe impl Layout for () {
  const SIZE: usize = 0;
}

unsafe impl Value for () {
  #[inline(always)]
  unsafe fn get(_: &[u8], _: &mut usize) -> Self {
  }

  #[inline(always)]
  unsafe fn set(_: &mut [u8], _: &mut usize, _: Self) {
  }
}

unsafe impl Layout for u8 {
  const SIZE: usize = 1;
}

unsafe impl Value for u8 {
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Layout for u16 {
  const SIZE: usize = 2;
}

unsafe impl Value for u16 {
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Layout for u32 {
  const SIZE: usize = 4;
}

unsafe impl Value for u32 {
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Layout for u64 {
  const SIZE: usize = 8;
}

unsafe impl Value for u64 {
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Layout for f32 {
  const SIZE: usize = 4;
}

unsafe impl Value for f32 {
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Layout for f64 {
  const SIZE: usize = 8;
}

unsafe impl Value for f64 {
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Layout for bool {
  const SIZE: usize = 1;
}

unsafe impl Value for bool {
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    0 != unsafe { u8::get(buf, ofs) }
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    unsafe { u8::set(buf, ofs, value as u8) }
  }
}

unsafe impl<T> Layout for Option<T>
where
  T: Value
{
  const SIZE: usize = 1 + T::SIZE;
}

unsafe impl<T> Value for Option<T>
where
  T: Value
{
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    if 0 == unsafe { u8::get(buf, ofs) } {
      None
    } else {
      Some(unsafe { T::get(buf, ofs) })
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

unsafe impl<T, E> Layout for Result<T, E>
where
  T: Value,
  E: Value
{
  const SIZE: usize = 1 + max(T::SIZE, E::SIZE);
}

unsafe impl<T, E> Value for Result<T, E>
where
  T: Value,
  E: Value
{
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    if 0 == unsafe { u8::get(buf, ofs) } {
      Ok(unsafe { T::get(buf, ofs) })
    } else {
      Err(unsafe { E::get(buf, ofs) })
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

unsafe impl<T, const N: usize> Layout for [T; N]
where
  T: Value
{
  const SIZE: usize = T::SIZE * N;
}

unsafe impl<T, const N: usize> Value for [T; N]
where
  T: Value
{
  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut usize) -> Self {
    core::array::from_fn(|_| unsafe { T::get(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut usize, value: Self) {
    value.iter().for_each(|&x| unsafe { T::set(buf, ofs, x) });
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

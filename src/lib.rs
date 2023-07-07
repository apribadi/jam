#![no_std]

pub mod internal;

use crate::internal::get_bytes;
use crate::internal::get_field;
use crate::internal::get_object;
use crate::internal::pop_slice;
use crate::internal::set_bytes;
use crate::internal::set_field;

/*
use crate::internal::get_slice;
use crate::internal::pop_slice;
*/

/// A `Value` is ...???
///
/// SAFETY:
///
/// A sound implementation of the `Value` trait must satisfy the following
/// properties.
///
/// - ???

pub unsafe trait Value: Copy {
  const FIELD_SIZE: usize;

  /// SAFETY:
  ///
  /// ???

  unsafe fn get(ptr: &mut *const u8) -> Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn set(ptr: &mut *mut u8, value: Self);
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

  unsafe fn new<'a>(ptr: *const u8, len: usize) -> &'a Self;
}

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
  unsafe fn new<'a>(ptr: *const u8, len: usize) -> &'a Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(core::slice::from_raw_parts(ptr, len)) }
  }
}

impl<T> ArrayV<T>
where
  T: Value
{
  const STRIDE: usize = max(T::FIELD_SIZE, 1);

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
    unsafe { get_field(&self.buf, i) }
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
    unsafe { set_field(&mut self.buf, i, value) }
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
      let x = unsafe { get_field(p, 0) };
      Some(x)
    }
  }
}

#[repr(transparent)]
pub struct ArrayO<T>
where
  T: Object
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: [u8],
}

unsafe impl<T> Object for ArrayO<T>
where
  T: Object
{
  #[inline(always)]
  unsafe fn new<'a>(ptr: *const u8, len: usize ) -> &'a Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(core::slice::from_raw_parts(ptr, len)) }
  }
}

impl<T> ArrayO<T>
where
  T: Object
{
  const ELT_SIZE: usize = core::mem::size_of::<T>();
  const STRIDE: usize = max(Self::ELT_SIZE, 1);

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
    let j = i + Self::ELT_SIZE;
    unsafe { get_object(&self.buf, i, j) }
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
  T: 'a + Object
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: &'a [u8],
}

impl<'a, T> Iterator for IterArrayO<'a, T>
where
  T: 'a + Object
{
  type Item = &'a T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.buf.is_empty() {
      None
    } else {
      let p = unsafe { pop_slice(&mut self.buf, ArrayO::<T>::STRIDE) };
      let x = unsafe { get_object(p, 0, ArrayO::<T>::ELT_SIZE) };
      Some(x)
    }
  }
}

unsafe impl Value for u8 {
  const FIELD_SIZE: usize = 1;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { set_bytes(ptr, value.to_le_bytes()) }
  }
}

unsafe impl Value for u16 {
  const FIELD_SIZE: usize = 2;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { set_bytes(ptr, value.to_le_bytes()) }
  }
}

unsafe impl Value for u32 {
  const FIELD_SIZE: usize = 4;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { set_bytes(ptr, value.to_le_bytes()) }
  }
}

unsafe impl Value for u64 {
  const FIELD_SIZE: usize = 8;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { set_bytes(ptr, value.to_le_bytes()) }
  }
}

/*

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
*/

#[inline(never)]
#[cold]
fn panic_out_of_bounds() -> ! {
  panic!()
}

#[inline(always)]
const fn max(x: usize, y: usize) -> usize {
  if x >= y { x } else { y }
}

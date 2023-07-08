#![no_std]

pub mod internal;

use crate::internal::get_bytes;
use crate::internal::get_field;
use crate::internal::get_value;
use crate::internal::pop_slice;
use crate::internal::set_bytes;

/// A `Value` is ...???
///
/// SAFETY:
///
/// A sound implementation of the `Value` trait must satisfy the following
/// properties.
///
/// - ???

pub unsafe trait Value: Copy {
  const STRIDE: usize;

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

  unsafe fn new(buf: &[u8]) -> &Self;
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
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(buf) }
  }
}

impl<T> ArrayV<T>
where
  T: Value
{
  const STRIDE: usize = max(T::STRIDE, 1);

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
    unsafe { get_value(&self.buf, i) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On index out of bounds.

  #[inline(always)]
  pub fn get(&self, index: usize) -> T {
    if index >= self.len() { panic_index_out_of_bounds() }
    unsafe { self.get_unchecked(index) }
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
      let x = unsafe { get_value(p, 0) };
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
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(buf) }
  }
}

impl<T> ArrayO<T>
where
  T: Object
{
  const SIZE: usize = core::mem::size_of::<T>();
  const STRIDE: usize = max(Self::SIZE, 1);

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
    unsafe { get_field(&self.buf, i, Self::SIZE) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On index out of bounds.

  #[inline(always)]
  pub fn get(&self, index: usize) -> &T {
    if index >= self.len() { panic_index_out_of_bounds() }
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
  T: Object + 'a
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: &'a [u8],
}

impl<'a, T> Iterator for IterArrayO<'a, T>
where
  T: Object + 'a
{
  type Item = &'a T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.buf.is_empty() {
      None
    } else {
      let p = unsafe { pop_slice(&mut self.buf, ArrayO::<T>::STRIDE) };
      let x = unsafe { get_field(p, 0, ArrayO::<T>::SIZE) };
      Some(x)
    }
  }
}

#[repr(transparent)]
pub struct ArrayI<T>
where
  T: Object + ?Sized
{
  _pd: core::marker::PhantomData<fn(T) -> T>,
  buf: [u8],
}

unsafe impl<T> Object for ArrayI<T>
where
  T: Object + ?Sized
{
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(buf) }
  }
}

impl<T> ArrayI<T>
where
  T: Object + ?Sized
{
  #[inline(always)]
  unsafe fn __ofs(&self, index: usize) -> usize {
    let i = 4 * (index - 3);
    let x = unsafe { get_value::<u32>(&self.buf, i) };
    x as usize
  }

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    let n = unsafe { self.__ofs(0) };
    n == 4
  }

  #[inline(always)]
  pub fn len(&self) -> usize {
    let n = unsafe { self.__ofs(0) };
    n / 4 - 1
  }

  /*
  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: usize) -> &T {
    let i = Self::STRIDE * index;
    unsafe { get_field(&self.buf, i, Self::SIZE) }
  }

  #[inline(always)]
  pub fn get(&self, index: usize) -> &T {
    if index >= self.len() { panic_index_out_of_bounds() }
    unsafe { self.get_unchecked(index) }
  }
  */
}

unsafe impl Value for u8 {
  const STRIDE: usize = 1;

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
  const STRIDE: usize = 2;

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
  const STRIDE: usize = 4;

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
  const STRIDE: usize = 8;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { set_bytes(ptr, value.to_le_bytes()) }
  }
}

unsafe impl Value for f32 {
  const STRIDE: usize = 4;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { set_bytes(ptr, value.to_le_bytes()) }
  }
}

unsafe impl Value for f64 {
  const STRIDE: usize = 8;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { set_bytes(ptr, value.to_le_bytes()) }
  }
}

unsafe impl Value for bool {
  const STRIDE: usize = 1;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    0 != unsafe { u8::get(ptr) }
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { u8::set(ptr, value as u8) }
  }
}

unsafe impl<T> Value for Option<T>
where
  T: Value
{
  const STRIDE: usize = 1 + T::STRIDE;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    if 0 == unsafe { u8::get(ptr) } {
      None
    } else {
      Some(unsafe { T::get(ptr) })
    }
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    match value {
      None => {
        unsafe { u8::set(ptr, 0) };
      }
      Some(x) => {
        unsafe { u8::set(ptr, 1) };
        unsafe { T::set(ptr, x) };
      }
    }
  }
}

unsafe impl<T, E> Value for Result<T, E>
where
  T: Value,
  E: Value
{
  const STRIDE: usize = 1 + max(T::STRIDE, E::STRIDE);

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    if 0 == unsafe { u8::get(ptr) } {
      Ok(unsafe { T::get(ptr) })
    } else {
      Err(unsafe { E::get(ptr) })
    }
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    match value {
      Ok(x) => {
        unsafe { u8::set(ptr, 0) };
        unsafe { T::set(ptr, x) };
      }
      Err(e) => {
        unsafe { u8::set(ptr, 1) };
        unsafe { E::set(ptr, e) };
      }
    }
  }
}

unsafe impl<T, U> Value for (T, U)
where
  T: Value,
  U: Value
{
  const STRIDE: usize = T::STRIDE + U::STRIDE;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    (unsafe { T::get(ptr) }, unsafe { U::get(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    unsafe { T::set(ptr, value.0) };
    unsafe { U::set(ptr, value.1) };
  }
}

unsafe impl<T, const N: usize> Value for [T; N]
where
  T: Value
{
  const STRIDE: usize = T::STRIDE * N;

  #[inline(always)]
  unsafe fn get(ptr: &mut *const u8) -> Self {
    core::array::from_fn(|_| unsafe { T::get(ptr) })
  }

  #[inline(always)]
  unsafe fn set(ptr: &mut *mut u8, value: Self) {
    value.iter().for_each(|&x| unsafe { T::set(ptr, x) });
  }
}

#[inline(never)]
#[cold]
fn panic_index_out_of_bounds() -> ! {
  panic!("index out of bounds!")
}

#[inline(always)]
const fn max(x: usize, y: usize) -> usize {
  if x >= y { x } else { y }
}

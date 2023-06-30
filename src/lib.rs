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

  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self);
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
  const STRIDE: u32 = max(T::SIZE, 1);

  /// ???

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.buf.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.buf.len() as u32 / Self::STRIDE
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, idx: u32) -> T {
    let i = Self::STRIDE as u64 * idx as u64;
    let n = self.buf.len() as u64;
    if i >= n { panic_out_of_bounds() }
    let i = i as u32;
    unsafe { T::get(&self.buf, &mut {i}) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, idx: u32) -> T {
    let i = Self::STRIDE * idx;
    unsafe { T::get(&self.buf, &mut {i}) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn set(&mut self, idx: u32, value: T) {
    let i = Self::STRIDE as u64 * idx as u64;
    let n = self.buf.len() as u64;
    if i >= n { panic_out_of_bounds() }
    let i = i as u32;
    unsafe { T::set(&mut self.buf, &mut {i}, value) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn set_unchecked(&mut self, idx: u32, value: T) {
    let i = Self::STRIDE * idx;
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
      let p = unsafe { pop_slice(&mut self.buf, ArrayV::<T>::STRIDE) };
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
  const STRIDE: u32 = max(T::SIZE, 1);

  /// ???

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.buf.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.buf.len() as u32 / Self::STRIDE
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, idx: u32) -> &T {
    let i = Self::STRIDE as u64 * idx as u64;
    let n = self.buf.len() as u64;
    if i >= n { panic_out_of_bounds() }
    let i = i as u32;
    unsafe { T::new(get_slice(&self.buf, i, Self::STRIDE)) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, idx: u32) -> &T {
    let i = Self::STRIDE * idx;
    unsafe { T::new(get_slice(&self.buf, i, Self::STRIDE)) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get_mut(&mut self, idx: u32) -> &mut T {
    let i = Self::STRIDE as u64 * idx as u64;
    let n = self.buf.len() as u64;
    if i >= n { panic_out_of_bounds() }
    let i = i as u32;
    unsafe { T::new_mut(get_slice_mut(&mut self.buf, i, Self::STRIDE)) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked_mut(&mut self, idx: u32) -> &mut T {
    let i = Self::STRIDE * idx;
    unsafe { T::new_mut(get_slice_mut(&mut self.buf, i, Self::STRIDE)) }
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
      let p = unsafe { pop_slice(&mut self.buf, ArrayO::<T>::STRIDE) };
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
      let p = unsafe { pop_slice_mut(&mut self.buf, ArrayO::<T>::STRIDE) };
      let x = unsafe { T::new_mut(p) };
      Some(x)
    }
  }
}

unsafe impl Value for () {
  const SIZE: u32 = 0;

  #[inline(always)]
  unsafe fn get(_: &[u8], _: &mut u32) -> Self {
  }

  #[inline(always)]
  unsafe fn set(_: &mut [u8], _: &mut u32, _: Self) {
  }
}

unsafe impl Value for u8 {
  const SIZE: u32 = 1;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for u16 {
  const SIZE: u32 = 2;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for u32 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for u64 {
  const SIZE: u32 = 8;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for f32 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for f64 {
  const SIZE: u32 = 8;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { get_bytes(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
    unsafe { set_bytes(buf, ofs, value.to_le_bytes()) }
  }
}

unsafe impl Value for bool {
  const SIZE: u32 = 1;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    match unsafe { u8::get(buf, ofs) } {
      0 => false,
      _ => true
    }
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
    unsafe { u8::set(buf, ofs, value as u8) }
  }
}

unsafe impl<T> Value for Option<T>
where
  T: Value
{
  const SIZE: u32 = 1 + T::SIZE;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    match unsafe { u8::get(buf, ofs) } {
      0 => None,
      _ => Some(unsafe { T::get(buf, ofs) })
    }
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
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
  const SIZE: u32 = 1 + max(T::SIZE, E::SIZE);

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    match unsafe { u8::get(buf, ofs) } {
      0 => Ok(unsafe { T::get(buf, ofs) }),
      _ => Err(unsafe { E::get(buf, ofs) })
    }
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
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
  const SIZE: u32 = T::SIZE * N as u32;

  #[inline(always)]
  unsafe fn get(buf: &[u8], ofs: &mut u32) -> Self {
    core::array::from_fn(|_| unsafe { T::get(buf, ofs) })
  }

  #[inline(always)]
  unsafe fn set(buf: &mut [u8], ofs: &mut u32, value: Self) {
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
const fn max(x: u32, y: u32) -> u32 {
  if x >= y { x } else { y }
}

#[inline(always)]
unsafe fn get_array<const N: usize>(buf: &[u8], ofs: u32) -> &[u8; N] {
  let p = buf.as_ptr();
  let p = unsafe { p.add(ofs as usize) };
  let p = p as *const [u8; N];
  unsafe { &*p }
}

#[inline(always)]
unsafe fn get_array_mut<const N: usize>(buf: &mut [u8], ofs: u32) -> &mut [u8; N] {
  let p = buf.as_mut_ptr();
  let p = unsafe { p.add(ofs as usize) };
  let p = p as *mut [u8; N];
  unsafe { &mut *p }
}

#[inline(always)]
unsafe fn get_slice(buf: &[u8], ofs: u32, len: u32) -> &[u8] {
  unsafe { buf.get_unchecked(ofs as usize .. (ofs + len) as usize) }
}

#[inline(always)]
unsafe fn get_slice_mut(buf: &mut [u8], ofs: u32, len: u32) -> &mut [u8] {
  unsafe { buf.get_unchecked_mut(ofs as usize .. (ofs + len) as usize) }
}

#[inline(always)]
unsafe fn pop_slice<'a>(buf: &mut &'a [u8], len: u32) -> &'a [u8] {
  let x = unsafe { buf.get_unchecked(.. len as usize) };
  let y = unsafe { buf.get_unchecked(len as usize ..) };
  *buf = y;
  x
}

#[inline(always)]
unsafe fn pop_slice_mut<'a>(buf: &mut &'a mut [u8], len: u32) -> &'a mut [u8] {
  let p = buf.as_mut_ptr();
  let a = len as usize;
  let b = buf.len() - a;
  let x = unsafe { core::slice::from_raw_parts_mut(p, a) };
  let y = unsafe { core::slice::from_raw_parts_mut(p.add(a), b) };
  *buf = y;
  x
}

#[inline(always)]
unsafe fn get_bytes<const N: usize>(buf: &[u8], ofs: &mut u32) -> [u8; N] {
  let p = unsafe { get_array(buf, *ofs) };
  *ofs += N as u32;
  *p
}

#[inline(always)]
unsafe fn set_bytes<const N: usize>(buf: &mut [u8], ofs: &mut u32, value: [u8; N]) {
  let p = unsafe { get_array_mut(buf, *ofs) };
  *ofs += N as u32;
  *p = value;
}

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

  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self);
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

  unsafe fn new(bytes: &[u8]) -> &Self;

  /// SAFETY:
  ///
  /// ???

  unsafe fn new_mut(bytes: &mut [u8]) -> &mut Self;
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
  bytes: [u8],
}

unsafe impl<T> Object for ArrayV<T>
where
  T: Value
{
  #[inline(always)]
  unsafe fn new(bytes: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(bytes) }
  }

  #[inline(always)]
  unsafe fn new_mut(bytes: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute::<&mut [u8], &mut Self>(bytes) }
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
    self.bytes.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.bytes.len() as u32 / Self::STRIDE
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: u32) -> T {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.bytes.len() as u64;

    if i >= n { panic_out_of_bounds() }

    let i = i as u32;

    unsafe { T::read(&self.bytes, &mut {i}) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: u32) -> T {
    let i = Self::STRIDE * index;
    unsafe { T::read(&self.bytes, &mut {i}) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn set(&mut self, index: u32, value: T) {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.bytes.len() as u64;

    if i >= n { panic_out_of_bounds() }

    let i = i as u32;

    unsafe { T::write(&mut self.bytes, &mut {i}, value) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn set_unchecked(&mut self, index: u32, value: T) {
    let i = Self::STRIDE * index;
    unsafe { T::write(&mut self.bytes, &mut {i}, value) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = T> {
    IterArrayV {
      _marker: core::marker::PhantomData,
      bytes: &self.bytes,
    }
  }
}

struct IterArrayV<'a, T>
where
  T: Value
{
  _marker: core::marker::PhantomData<fn(T) -> T>,
  bytes: &'a [u8],
}

impl<'a, T> Iterator for IterArrayV<'a, T>
where
  T: Value
{
  type Item = T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.bytes.is_empty() {
      None
    } else {
      let p = unsafe { pop_slice(&mut self.bytes, ArrayV::<T>::STRIDE) };
      let x = unsafe { T::read(p, &mut {0}) };
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
  bytes: [u8],
}

unsafe impl<T> Object for ArrayO<T>
where
  T: ?Sized + SizedObject
{
  #[inline(always)]
  unsafe fn new(bytes: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(bytes) }
  }

  #[inline(always)]
  unsafe fn new_mut(bytes: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute::<&mut [u8], &mut Self>(bytes) }
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
    self.bytes.is_empty()
  }

  /// ???

  #[inline(always)]
  pub fn len(&self) -> u32 {
    self.bytes.len() as u32 / Self::STRIDE
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get(&self, index: u32) -> &T {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.bytes.len() as u64;
    if i >= n { panic_out_of_bounds() }
    let i = i as u32;
    unsafe { T::new(get_slice(&self.bytes, i, Self::STRIDE)) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked(&self, index: u32) -> &T {
    let i = Self::STRIDE * index;
    unsafe { T::new(get_slice(&self.bytes, i, Self::STRIDE)) }
  }

  /// ???
  ///
  /// PANICS:
  ///
  /// On out-of-bounds access.

  #[inline(always)]
  pub fn get_mut(&mut self, index: u32) -> &mut T {
    let i = Self::STRIDE as u64 * index as u64;
    let n = self.bytes.len() as u64;
    if i >= n { panic_out_of_bounds() }
    let i = i as u32;
    unsafe { T::new_mut(get_slice_mut(&mut self.bytes, i, Self::STRIDE)) }
  }

  /// ???
  ///
  /// SAFETY:
  ///
  /// ???

  #[inline(always)]
  pub unsafe fn get_unchecked_mut(&mut self, index: u32) -> &mut T {
    let i = Self::STRIDE * index;
    unsafe { T::new_mut(get_slice_mut(&mut self.bytes, i, Self::STRIDE)) }
  }

  /// ???

  #[inline(always)]
  pub fn iter(&self) -> impl '_ + Iterator<Item = &'_ T> {
    IterArrayO {
      _marker: core::marker::PhantomData,
      bytes: &self.bytes,
    }
  }

  #[inline(always)]
  unsafe fn pop_unchecked_mut(&mut self) -> (&mut T, &mut Self) {
    // let x = unsafe { self.bytes.get_unchecked_mut(Self::STRIDE as usize ..) };
    // let y = unsafe { self.bytes.get_unchecked_mut(.. Self::STRIDE as usize) };
    let (x, y) = self.bytes.split_at_mut(Self::STRIDE as usize);
    let x = unsafe { T::new_mut(x) };
    let y = unsafe { Self::new_mut(y) };
    (x, y)
  }
}

struct IterArrayO<'a, T>
where
  T: ?Sized + SizedObject
{
  _marker: core::marker::PhantomData<fn(T) -> T>,
  bytes: &'a [u8],
}

impl<'a, T> Iterator for IterArrayO<'a, T>
where
  T: 'a + ?Sized + SizedObject
{
  type Item = &'a T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.bytes.is_empty() {
      None
    } else {
      let p = unsafe { pop_slice(&mut self.bytes, ArrayO::<T>::STRIDE) };
      let x = unsafe { T::new(p) };
      Some(x)
    }
  }
}

struct IterMutArrayO<'a, T>(&'a mut ArrayO<T>)
where
  T: ?Sized + SizedObject;

impl<'a, T> Iterator for IterMutArrayO<'a, T>
where
  T: ?Sized + SizedObject
{
  type Item = &'a mut T;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() {
      None
    } else {
      /*
      let (x, y) = unsafe { self.0.pop_unchecked_mut() };
      self.0 = y;
      Some(x)
      */
      panic!()
    }
  }
}

unsafe impl Value for () {
  const SIZE: u32 = 0;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    let _ = bytes;
    let _ = offset;
    ()
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    let _ = bytes;
    let _ = offset;
    let _ = value;
  }
}

unsafe impl Value for u8 {
  const SIZE: u32 = 1;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(bytes, offset) })
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(bytes, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for u16 {
  const SIZE: u32 = 2;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(bytes, offset) })
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(bytes, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for u32 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(bytes, offset) })
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(bytes, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for u64 {
  const SIZE: u32 = 8;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(bytes, offset) })
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(bytes, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for f32 {
  const SIZE: u32 = 4;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(bytes, offset) })
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(bytes, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for f64 {
  const SIZE: u32 = 8;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    Self::from_le_bytes(unsafe { read_chunk(bytes, offset) })
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { write_chunk(bytes, offset, value.to_le_bytes()) }
  }
}

unsafe impl Value for bool {
  const SIZE: u32 = 1;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    match unsafe { u8::read(bytes, offset) } {
      0 => false,
      _ => true
    }
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    unsafe { u8::write(bytes, offset, value as u8) }
  }
}

unsafe impl<T> Value for Option<T>
where
  T: Value
{
  const SIZE: u32 = 1 + T::SIZE;

  #[inline(always)]
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    match unsafe { u8::read(bytes, offset) } {
      0 => None,
      _ => Some(unsafe { T::read(bytes, offset) })
    }
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    match value {
      None => {
        unsafe { u8::write(bytes, offset, 0) };
      }
      Some(x) => {
        unsafe { u8::write(bytes, offset, 1) };
        unsafe { T::write(bytes, offset, x) };
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
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    match unsafe { u8::read(bytes, offset) } {
      0 => Ok(unsafe { T::read(bytes, offset) }),
      _ => Err(unsafe { E::read(bytes, offset) })
    }
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    match value {
      Ok(x) => {
        unsafe { u8::write(bytes, offset, 0) };
        unsafe { T::write(bytes, offset, x) };
      }
      Err(e) => {
        unsafe { u8::write(bytes, offset, 1) };
        unsafe { E::write(bytes, offset, e) };
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
  unsafe fn read(bytes: &[u8], offset: &mut u32) -> Self {
    core::array::from_fn(|_| unsafe { T::read(bytes, offset) })
  }

  #[inline(always)]
  unsafe fn write(bytes: &mut [u8], offset: &mut u32, value: Self) {
    for &item in value.iter() {
      unsafe { T::write(bytes, offset, item) }
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
unsafe fn get_chunk<const N: usize>(bytes: &[u8], offset: u32) -> &[u8; N] {
  let p = bytes.as_ptr();
  let p = unsafe { p.add(offset as usize) };
  let p = p as *const [u8; N];
  unsafe { &*p }
}

#[inline(always)]
unsafe fn get_chunk_mut<const N: usize>(bytes: &mut [u8], offset: u32) -> &mut [u8; N] {
  let p = bytes.as_mut_ptr();
  let p = unsafe { p.add(offset as usize) };
  let p = p as *mut [u8; N];
  unsafe { &mut *p }
}

#[inline(always)]
unsafe fn get_slice(bytes: &[u8], offset: u32, length: u32) -> &[u8] {
  let i = offset as usize;
  let j = (offset + length) as usize;
  unsafe { bytes.get_unchecked(i .. j) }
}

#[inline(always)]
unsafe fn get_slice_mut(bytes: &mut [u8], offset: u32, length: u32) -> &mut [u8] {
  let i = offset as usize;
  let j = (offset + length) as usize;
  unsafe { bytes.get_unchecked_mut(i .. j) }
}

#[inline(always)]
unsafe fn pop_slice<'a, 'b>(bytes: &'a mut &'b [u8], length: u32) -> &'b [u8] {
  let i = length as usize;
  let x = unsafe { bytes.get_unchecked(.. i) };
  let y = unsafe { bytes.get_unchecked(.. i) };
  *bytes = y;
  x
}

/*
#[inline(always)]
unsafe fn split_at(bytes: &[u8], i: u32) -> (&[u8], &[u8]) {
  unsafe { (bytes.get_unchecked(.. i as usize), bytes.get_unchecked(i as usize ..)) }
}
*/

#[inline(always)]
unsafe fn read_chunk<const N: usize>(bytes: &[u8], offset: &mut u32) -> [u8; N] {
  let p = unsafe { get_chunk(bytes, *offset) };
  *offset += N as u32;
  *p
}

#[inline(always)]
unsafe fn write_chunk<const N: usize>(bytes: &mut [u8], offset: &mut u32, value: [u8; N]) {
  let p = unsafe { get_chunk_mut(bytes, *offset) };
  *offset += N as u32;
  *p = value;
}

use crate::Object;
use crate::Value;

#[inline(always)]
pub(crate) unsafe fn get_slice(buf: &[u8], ofs: usize, len: usize) -> &[u8] {
  let p = buf.as_ptr().wrapping_add(ofs);
  unsafe { core::slice::from_raw_parts(p, len) }
}

#[inline(always)]
pub(crate) unsafe fn pop_slice<'a>(buf: &mut &'a [u8], ofs: usize) -> &'a [u8] {
  let p = buf.as_ptr();
  let n = buf.len();
  let q = p.wrapping_add(ofs);
  let x = unsafe { core::slice::from_raw_parts(p, ofs) };
  let y = unsafe { core::slice::from_raw_parts(q, n - ofs) };
  *buf = y;
  x
}

#[inline(always)]
pub(crate) unsafe fn get_bytes<const N: usize>(ptr: &mut *const u8) -> [u8; N] {
  let p = *ptr;
  let q = p.wrapping_add(N);
  let p = p as *const [u8; N];
  let x = unsafe { core::ptr::read(p) };
  *ptr = q;
  x
}

#[inline(always)]
pub(crate) unsafe fn set_bytes<const N: usize>(ptr: &mut *mut u8, value: [u8; N]) {
  let p = *ptr;
  let q = p.wrapping_add(N);
  let p = p as *mut [u8; N];
  unsafe { core::ptr::write(p, value) };
  *ptr = q;
}

#[inline(always)]
pub const fn stride_of<T>() -> usize
where
  T: Value
{
  T::STRIDE
}

#[inline(always)]
pub unsafe fn get_value<T>(ptr: &mut *const u8) -> T
where
  T: Value
{
  unsafe { T::get(ptr) }
}

#[inline(always)]
pub unsafe fn set_value<T>(ptr: &mut *mut u8, value: T)
where
  T: Value
{
  unsafe { T::set(ptr, value) }
}

#[inline(always)]
pub unsafe fn get_field<T>(buf: &[u8], ofs: usize) -> T
where
  T: Value
{
  let p = buf.as_ptr().wrapping_add(ofs);
  unsafe { T::get(&mut {p}) }
}

#[inline(always)]
pub unsafe fn get_child<T>(buf: &[u8], ofs: usize, len: usize) -> &T
where
  T: Object + ?Sized
{
  unsafe { T::new(get_slice(buf, ofs, len)) }
}

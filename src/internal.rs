use crate::Object;
use crate::Value;

#[inline(always)]
pub unsafe fn get_array<const N: usize>(buf: &[u8], ofs: usize) -> &[u8; N] {
  let p = buf.as_ptr().wrapping_add(ofs);
  let p = p as *const [u8; N];
  unsafe { &*p}
}

#[inline(always)]
pub unsafe fn get_array_mut<const N: usize>(buf: &mut [u8], ofs: usize) -> &mut [u8; N] {
  let p = buf.as_mut_ptr().wrapping_add(ofs);
  let p = p as *mut [u8; N];
  unsafe { &mut *p }
}

#[inline(always)]
pub unsafe fn get_slice(buf: &[u8], ofs: usize, len: usize) -> &[u8] {
  let p = buf.as_ptr().wrapping_add(ofs);
  unsafe { core::slice::from_raw_parts(p, len) }
}

#[inline(always)]
pub unsafe fn get_slice_mut(buf: &mut [u8], ofs: usize, len: usize) -> &mut [u8] {
  let p = buf.as_mut_ptr().wrapping_add(ofs);
  unsafe { core::slice::from_raw_parts_mut(p, len) }
}

#[inline(always)]
pub unsafe fn pop_slice<'a>(buf: &mut &'a [u8], ofs: usize) -> &'a [u8] {
  let x = unsafe { buf.get_unchecked(.. ofs) };
  let y = unsafe { buf.get_unchecked(ofs ..) };
  *buf = y;
  x
}

#[inline(always)]
pub unsafe fn pop_slice_mut<'a>(buf: &mut &'a mut [u8], ofs: usize) -> &'a mut [u8] {
  let p = buf.as_mut_ptr();
  let a = ofs;
  let q = p.wrapping_add(ofs);
  let b = buf.len() - ofs;
  let x = unsafe { core::slice::from_raw_parts_mut(p, a) };
  let y = unsafe { core::slice::from_raw_parts_mut(q, b) };
  *buf = y;
  x
}

#[inline(always)]
pub unsafe fn get_bytes<const N: usize>(ptr: &mut *const u8) -> [u8; N] {
  let p = *ptr;
  let q = p.wrapping_add(N);
  let p = p as *const [u8; N];
  let x = unsafe { core::ptr::read(p) };
  *ptr = q;
  x
}

#[inline(always)]
pub unsafe fn set_bytes<const N: usize>(ptr: &mut *mut u8, value: [u8; N]) {
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
pub unsafe fn get_value<T>(buf: &[u8], ofs: usize) -> T
where
  T: Value
{
  let p = buf.as_ptr().wrapping_add(ofs);
  unsafe { T::get(&mut {p}) }
}

#[inline(always)]
pub unsafe fn set_value<T>(buf: &mut [u8], ofs: usize, value: T)
where
  T: Value
{
  let p = buf.as_mut_ptr().wrapping_add(ofs);
  unsafe { T::set(&mut {p}, value) }
}

#[inline(always)]
pub unsafe fn get_field<T>(buf: &[u8], ofs: usize, len: usize) -> &T
where
  T: Object + ?Sized
{
  unsafe { T::new(get_slice(buf, ofs, len)) }
}

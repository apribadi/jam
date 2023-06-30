use crate::Object;
use crate::SizedObject;
use crate::Value;

#[inline(always)]
pub unsafe fn get_array<const N: usize>(buf: &[u8], ofs: u32) -> &[u8; N] {
  let p = buf.as_ptr();
  let p = unsafe { p.add(ofs as usize) };
  let p = p as *const [u8; N];
  unsafe { &*p }
}

#[inline(always)]
pub unsafe fn get_array_mut<const N: usize>(buf: &mut [u8], ofs: u32) -> &mut [u8; N] {
  let p = buf.as_mut_ptr();
  let p = unsafe { p.add(ofs as usize) };
  let p = p as *mut [u8; N];
  unsafe { &mut *p }
}

#[inline(always)]
pub unsafe fn get_slice(buf: &[u8], lo: u32, hi: u32) -> &[u8] {
  unsafe { buf.get_unchecked(lo as usize .. hi as usize) }
}

#[inline(always)]
pub unsafe fn get_slice_mut(buf: &mut [u8], lo: u32, hi: u32) -> &mut [u8] {
  unsafe { buf.get_unchecked_mut(lo as usize .. hi as usize) }
}

#[inline(always)]
pub unsafe fn pop_slice<'a>(buf: &mut &'a [u8], ofs: u32) -> &'a [u8] {
  let x = unsafe { buf.get_unchecked(.. ofs as usize) };
  let y = unsafe { buf.get_unchecked(ofs as usize ..) };
  *buf = y;
  x
}

#[inline(always)]
pub unsafe fn pop_slice_mut<'a>(buf: &mut &'a mut [u8], ofs: u32) -> &'a mut [u8] {
  let p = buf.as_mut_ptr();
  let a = ofs as usize;
  let b = buf.len() - a;
  let x = unsafe { core::slice::from_raw_parts_mut(p, a) };
  let y = unsafe { core::slice::from_raw_parts_mut(p.add(a), b) };
  *buf = y;
  x
}

#[inline(always)]
pub unsafe fn get_bytes<const N: usize>(buf: &[u8], ofs: &mut u32) -> [u8; N] {
  let p = unsafe { get_array(buf, *ofs) };
  *ofs += N as u32;
  *p
}

#[inline(always)]
pub unsafe fn set_bytes<const N: usize>(buf: &mut [u8], ofs: &mut u32, value: [u8; N]) {
  let p = unsafe { get_array_mut(buf, *ofs) };
  *ofs += N as u32;
  *p = value;
}

#[inline(always)]
pub unsafe fn get_u32(buf: &[u8], ofs: u32) -> u32 {
  unsafe { u32::get(buf, &mut {ofs}) }
}

#[inline(always)]
pub unsafe fn get_value<T>(buf: &[u8], ofs: &mut u32) -> T
where
  T: Value
{
  unsafe { T::get(buf, ofs) }
}

#[inline(always)]
pub unsafe fn get_object<T>(buf: &[u8], lo: u32, hi: u32) -> &T
where
  T: ?Sized + Object
{
  unsafe { T::new(get_slice(buf, lo, hi)) }
}

#[inline(always)]
pub const fn size_of_value<T>() -> u32
where
  T: Value
{
  T::SIZE
}

#[inline(always)]
pub const fn size_of_object<T>() -> u32
where
  T: ?Sized + SizedObject
{
  T::SIZE
}

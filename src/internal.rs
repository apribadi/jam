use crate::Object;
use crate::SizeOf;
use crate::Value;

#[inline(always)]
pub unsafe fn get_array<const N: usize>(buf: &[u8], ofs: usize) -> &[u8; N] {
  let p = buf.as_ptr();
  let p = unsafe { p.add(ofs) };
  let p = p as *const [u8; N];
  unsafe { &*p }
}

#[inline(always)]
pub unsafe fn get_array_mut<const N: usize>(buf: &mut [u8], ofs: usize) -> &mut [u8; N] {
  let p = buf.as_mut_ptr();
  let p = unsafe { p.add(ofs) };
  let p = p as *mut [u8; N];
  unsafe { &mut *p }
}

#[inline(always)]
pub unsafe fn get_slice(buf: &[u8], lo: usize, hi: usize) -> &[u8] {
  unsafe { buf.get_unchecked(lo .. hi) }
}

#[inline(always)]
pub unsafe fn get_slice_mut(buf: &mut [u8], lo: usize, hi: usize) -> &mut [u8] {
  unsafe { buf.get_unchecked_mut(lo .. hi) }
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
  let b = buf.len() - a;
  let x = unsafe { core::slice::from_raw_parts_mut(p, a) };
  let y = unsafe { core::slice::from_raw_parts_mut(p.add(a), b) };
  *buf = y;
  x
}

#[inline(always)]
pub unsafe fn get_bytes<const N: usize>(buf: &[u8], ofs: &mut usize) -> [u8; N] {
  let p = unsafe { get_array(buf, *ofs) };
  *ofs += N;
  *p
}

#[inline(always)]
pub unsafe fn set_bytes<const N: usize>(buf: &mut [u8], ofs: &mut usize, value: [u8; N]) {
  let p = unsafe { get_array_mut(buf, *ofs) };
  *ofs += N;
  *p = value;
}

#[inline(always)]
pub unsafe fn get_offset(buf: &[u8], ofs: usize) -> usize {
  (unsafe { u32::from_le_bytes(*get_array(buf, ofs)) }) as usize
}

#[inline(always)]
pub const fn size_of<T>() -> usize
where
  T: ?Sized + SizeOf
{
  T::SIZE
}

#[inline(always)]
pub unsafe fn get_value<T>(buf: &[u8], ofs: &mut usize) -> T
where
  T: Value
{
  unsafe { T::get(buf, ofs) }
}

#[inline(always)]
pub unsafe fn get_object<T>(buf: &[u8], lo: usize, hi: usize) -> &T
where
  T: ?Sized + Object
{
  unsafe { T::new(get_slice(buf, lo, hi)) }
}

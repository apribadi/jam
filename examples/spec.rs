// struct Foo
//   a U32
//   b U32
// end

#[repr(C)]
pub struct Foo {
  __a: [u8; <u32 as ::jam::Value>::STRIDE],
  __b: [u8; <u32 as ::jam::Value>::STRIDE],
}

unsafe impl ::jam::Object for Foo {
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { &*(buf.as_ptr() as *const Self) }
  }
}

impl Foo {
  pub fn a(&self) -> u32 {
    unsafe { <u32 as ::jam::Value>::get(&mut {&self.__a as *const u8}) }
  }

  pub fn b(&self) -> u32 {
    unsafe { <u32 as ::jam::Value>::get(&mut {&self.__b as *const u8}) }
  }
}

// struct Bar
//   a Foo
//   b U64
//   c Array[Foo]
//   d Array[U64]
// end

#[repr(C)]
pub struct Bar {
  __a: Foo,
  __b: [u8; <u64 as ::jam::Value>::STRIDE],
  ofs: [[u8; 4]; 1],
  fam: [u8],
}

unsafe impl ::jam::Object for Bar {
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    let p = buf.as_ptr();
    let n = buf.len();
    let q = ::core::ptr::slice_from_raw_parts(p, 0) as *const Self;
    let m = ::core::mem::size_of_val(unsafe { &*q });
    let q = ::core::ptr::slice_from_raw_parts(p, n - m) as *const Self;
    unsafe { &*q }
  }
}

impl Bar {
  pub fn a(&self) -> &Foo {
    &self.__a
  }

  pub fn b(&self) -> u64 {
    unsafe { <u64 as ::jam::Value>::get(&mut {&self.__b as *const u8}) }
  }

  pub fn c_(&self) -> &::jam::ArrayO<Foo> {
    let i = 0;
    let j = u32::from_le_bytes(self.ofs[0]) as usize;
    unsafe { ::jam::internal::get_child(&self.fam, i, j - i) }
  }

  pub fn c(&self) -> &[Foo] {
    let i = 0;
    let j = u32::from_le_bytes(self.ofs[0]) as usize;
    let n = (j - i) / ::core::mem::size_of::<Foo>();
    let p = self.fam.as_ptr().wrapping_add(i) as *const Foo;
    unsafe { ::core::slice::from_raw_parts(p, n) }
  }

  pub fn d(&self) -> &::jam::ArrayV<u64> {
    let i = u32::from_le_bytes(self.ofs[0]) as usize;
    let j = self.fam.len();
    unsafe { ::jam::internal::get_child(&self.fam, i, j - i) }
  }
}

// choice Baz
//   A(Foo)
//   B(Bar)
// end

pub enum Baz<'a> {
  A(&'a Foo),
  B(&'a Bar),
}

impl<'a> Baz<'a> {
  pub fn new(buf: &'a [u8]) -> Self {
    match unsafe { buf.get_unchecked(0) } {
      0 => Self::A(unsafe { ::jam::internal::get_child(buf, 1, buf.len() - 1) }),
      1 => Self::B(unsafe { ::jam::internal::get_child(buf, 1, buf.len() - 1) }),
      _ => unsafe { ::core::hint::unreachable_unchecked() },
    }
  }
}

pub fn foo(x: &::jam::ArrayV<u32>) -> u32 {
  let mut n = 0;
  for y in x.iter() {
    n += y;
  }
  n
}

pub fn bar(x: &::jam::ArrayO<Foo>) -> u32 {
  let mut n = 0;
  for y in x.iter() {
    n += y.a();
    n += y.b();
  }
  n
}

::jam::include!();

// START OF GENERATED CODE

// struct Foo
//   a U32
//   b U32
// end

#[repr(transparent)]
pub struct Foo([u8; Self::SIZE]);

unsafe impl jam::Object for Foo {
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    let p = buf.as_ptr();
    let p = p as *const Self;
    unsafe { &*p }
  }
}

impl Foo {
  const OFS0: usize = 0;
  const OFS1: usize = Self::OFS0 + jam::internal::stride_of::<u32>();
  const OFS2: usize = Self::OFS1 + jam::internal::stride_of::<u32>();
  const SIZE: usize = Self::OFS2;

  pub fn a(&self) -> u32 {
    let i = Self::OFS0;
    unsafe { jam::internal::get_value(&self.0, i) }
  }

  pub fn b(&self) -> u32 {
    let i = Self::OFS1;
    unsafe { jam::internal::get_value(&self.0, i) }
  }
}

// struct Bar
//   a Foo
//   b U64
//   c Array[Foo]
//   d Array[U64]
// end
//
// k : 0 .. 4
// a : 4 .. 12
// b : 12 .. 20
// c : 20 .. 20 + k
// d : 20 + k ..

#[repr(transparent)]
pub struct Bar([u8]);

unsafe impl jam::Object for Bar {
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(buf) }
  }
}

impl Bar {
  const OFS0: usize = 4;
  const OFS1: usize = Self::OFS0 + core::mem::size_of::<Foo>();
  const OFS2: usize = Self::OFS1 + jam::internal::stride_of::<u64>();

  #[inline(always)]
  unsafe fn __ofs(&self, index: usize) -> usize {
    let i = 4 * (index - 3);
    let x = unsafe { jam::internal::get_value::<u32>(&self.0, i) };
    x as usize
  }

  pub fn a(&self) -> &Foo {
    let i = Self::OFS0;
    let j = Self::OFS1;
    unsafe { jam::internal::get_field(&self.0, i, j - i) }
  }

  pub fn b(&self) -> u64 {
    let i = Self::OFS1;
    unsafe { jam::internal::get_value(&self.0, i) }
  }

  pub fn c(&self) -> &jam::ArrayO<Foo> {
    let i = Self::OFS2;
    let j = unsafe { self.__ofs(3) };
    unsafe { jam::internal::get_field(&self.0, i, j - i) }
  }

  pub fn d(&self) -> &jam::ArrayV<u64> {
    let i = unsafe { self.__ofs(3) };
    let j = unsafe { self.__ofs(4) };
    unsafe { jam::internal::get_field(&self.0, i, j - i) }
  }
}

pub fn foo(x: &jam::ArrayV<u32>) -> u32 {
  let mut n = 0;
  for y in x.iter() {
    n += y;
  }
  n
}

pub fn bar(x: &jam::ArrayO<Foo>) -> u32 {
  let mut n = 0;
  for y in x.iter() {
    n += y.a();
    n += y.b();
  }
  n
}

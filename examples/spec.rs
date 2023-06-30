// struct Foo
//   a U32
//   b U32
// end

#[repr(transparent)]
pub struct Foo([u8]);

impl Foo {
  const OFS0: u32 = 0;
  const OFS1: u32 = Self::OFS0 + jam::rt::size_of_value::<u32>();
  const OFS2: u32 = Self::OFS1 + jam::rt::size_of_value::<u32>();

  pub fn a(&self) -> u32 {
    let i = Self::OFS0;
    unsafe { jam::rt::get_value(&self.0, &mut {i}) }
  }

  pub fn b(&self) -> u32 {
    let i = Self::OFS2;
    unsafe { jam::rt::get_value(&self.0, &mut {i}) }
  }
}

unsafe impl jam::Object for Foo {
  #[inline(always)]
  unsafe fn new(buf: &[u8]) -> &Self {
    unsafe { core::mem::transmute(buf) }
  }

  #[inline(always)]
  unsafe fn new_mut(buf: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute(buf) }
  }
}

unsafe impl jam::SizedObject for Foo {
  const SIZE: u32 = Self::OFS2;
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

impl Bar {
  const OFS0: u32 = 4;
  const OFS1: u32 = Self::OFS0 + jam::rt::size_of_object::<Foo>();
  const OFS2: u32 = Self::OFS1 + jam::rt::size_of_value::<u64>();

  #[inline(always)]
  fn ofs3(&self) -> u32 {
    unsafe { jam::rt::get_u32(&self.0, 4 * 0) }
  }

  #[inline(always)]
  fn ofs4(&self) -> u32 {
    self.0.len() as u32
  }

  pub fn a(&self) -> &Foo {
    let i = Self::OFS0;
    let j = Self::OFS1;
    unsafe { jam::rt::get_object(&self.0, i, j) }
  }

  pub fn b(&self) -> u64 {
    let i = Self::OFS1;
    unsafe { jam::rt::get_value(&self.0, &mut {i}) }
  }

  pub fn c(&self) -> &jam::ArrayO<Foo> {
    let i = Self::OFS2;
    let j = self.ofs3();
    unsafe { jam::rt::get_object(&self.0, i, j) }
  }

  pub fn d(&self) -> &jam::ArrayV<u64> {
    let i = self.ofs3();
    let j = self.ofs4();
    unsafe { jam::rt::get_object(&self.0, i, j) }
  }
}

pub fn foo(x: &jam::ArrayV<u32>, i: u32) -> u32 {
  x.get(i)
}

pub fn bar(x: &jam::ArrayV<u32>) -> u32 {
  let mut n = 0;
  for y in x.iter() {
    n += y;
  }
  n
}

pub fn baz(x: &jam::ArrayO<Foo>) -> u32 {
  let mut n = 0;

  for y in x.iter() {
    n += y.a();
    n += y.b();
  }

  n
}

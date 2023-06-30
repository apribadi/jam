// struct Foo
//   a U32
//   b U32
// end

#[repr(transparent)]
pub struct Foo([u8]);

impl Foo {
  const OFS0: u32 = 0;
  const OFS1: u32 = Self::OFS0 + <u32 as jam::Value>::SIZE;
  const OFS2: u32 = Self::OFS1 + <u32 as jam::Value>::SIZE;

  pub fn a(&self) -> u32 {
    unsafe { <u32 as jam::Value>::read(&self.0, &mut {Self::OFS0}) }
  }

  pub fn b(&self) -> u32 {
    unsafe { <u32 as jam::Value>::read(&self.0, &mut {Self::OFS1}) }
  }
}

unsafe impl jam::Object for Foo {
  #[inline(always)]
  unsafe fn new(slice: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &Self>(slice) }
  }

  #[inline(always)]
  unsafe fn new_mut(slice: &mut [u8]) -> &mut Self {
    unsafe { core::mem::transmute::<&mut [u8], &mut Self>(slice) }
  }
}

unsafe impl jam::SizedObject for Foo {
  const SIZE: u32 = Self::OFS2;
}

/*

// Array[Foo]

#[repr(transparent)]
pub struct ArrayFoo([u8]);

impl jam::runtime::AsReprRef<[u8]> for ArrayFoo {
  #[inline(always)]
  fn as_repr_ref(&self) -> &[u8] {
    &self.0
  }
}

unsafe impl jam::runtime::OfReprRef<[u8]> for ArrayFoo {
  #[inline(always)]
  unsafe fn of_repr_ref(x: &[u8]) -> &Self {
    unsafe { core::mem::transmute::<&[u8], &ArrayFoo>(x) }
  }
}

impl ArrayFoo {
  pub fn len(&self) -> usize {
    self.0.len() / 8
  }

  pub fn get(&self, index: usize) -> &Foo {
    if ! (index < self.len()) { jam::runtime::panic_index_out_of_bounds() }

    let i = 8 * index;
    unsafe { jam::runtime::unchecked::get_ref::<_, _, _, 8>(self, i) }
  }

  pub unsafe fn get_unchecked(&self, index: usize) -> &Foo {
    let i = 8 * index;
    unsafe { jam::runtime::unchecked::get_ref::<_, _, _, 8>(self, i) }
  }
}

// Array[U32]

#[repr(transparent)]
pub struct ArrayU32([u8]);

impl jam::runtime::AsReprRef<[u8]> for ArrayU32 {
  #[inline(always)]
  fn as_repr_ref(&self) -> &[u8] {
    &self.0
  }
}

impl ArrayU32 {
  pub fn len(&self) -> usize {
    self.0.len() / 4
  }

  pub fn get(&self, index: usize) -> u32 {
    if ! (index < self.len()) { jam::runtime::panic_index_out_of_bounds() }

    let i = 4 * index;
    unsafe { jam::runtime::unchecked::get_value::<_, _, u32, 4>(self, i) }
  }

  pub unsafe fn get_unchecked(&self, index: usize) -> u32 {
    let i = 4 * index;
    unsafe { jam::runtime::unchecked::get_value::<_, _, u32, 4>(self, i) }
  }
}

// struct Bar
//   a Foo
//   b U64
//   c Array[Foo]
//   d Array[U32]
// end
//
// k : 0 .. 4
// a : 4 .. 12
// b : 12 .. 20
// c : 20 .. 20 + k
// d : 20 + k ..

#[repr(transparent)]
pub struct Bar([u8]);

impl jam::runtime::AsReprRef<[u8]> for Bar {
  #[inline(always)]
  fn as_repr_ref(&self) -> &[u8] {
    &self.0
  }
}

impl Bar {
  pub fn a(&self) -> &Foo {
    unsafe { jam::runtime::unchecked::get_ref::<_, _, _, 8>(self, 0) }
  }

  pub fn b(&self) -> u64 {
    unsafe { jam::runtime::unchecked::get_value::<_, _, _, 8>(self, 8) }
  }

  /*
  pub fn c(&self) -> &ArrayFoo {
    let i = 20;
    let j = 20 + unsafe { jam::runtime::unchecked::get_value::<_, _, u32, 4>
    let i = 16;
    let j = self.0.len();
    unsafe { jam::runtime::unchecked::get_unsized_ref(self, i, j) }
  }
  */
}
*/

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

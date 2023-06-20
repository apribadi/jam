// struct Foo
//   a U32
//   b U32
// end

#[repr(transparent)]
pub struct Foo([u8; 8]);

impl Foo {
  pub fn a(&self) -> u32 {
    let i = 0;
    let p = unsafe { jam::runtime::unchecked::get_array::<4>(&self.0, i) };
    jam::runtime::decode_u32(*p)
  }

  pub fn b(&self) -> u32 {
    let i = 4;
    let p = unsafe { jam::runtime::unchecked::get_array::<4>(&self.0, i) };
    jam::runtime::decode_u32(*p)
  }
}

#[repr(transparent)]
pub struct ArrayFoo([u8]);

impl ArrayFoo {
  pub fn len(&self) -> usize {
    self.0.len() / 8
  }

  pub fn get(&self, index: usize) -> &Foo {
    if ! (index < self.len()) { jam::runtime::panic_index_out_of_bounds() }

    let i = 8 * index;
    let p = unsafe { jam::runtime::unchecked::get_array::<8>(&self.0, i) };
    unsafe { core::mem::transmute::<&[u8; 8], &Foo>(p) }
  }

  pub unsafe fn get_unchecked(&self, index: usize) -> &Foo {
    let i = 8 * index;
    let p = unsafe { jam::runtime::unchecked::get_array::<8>(&self.0, i) };
    unsafe { core::mem::transmute::<&[u8; 8], &Foo>(p) }
  }
}

#[repr(transparent)]
pub struct ArrayU32([u8]);

impl ArrayU32 {
  pub fn len(&self) -> usize {
    self.0.len() / 4
  }

  pub fn get(&self, index: usize) -> u32 {
    if ! (index < self.len()) { jam::runtime::panic_index_out_of_bounds() }

    let i = 4 * index;
    let p = unsafe { jam::runtime::unchecked::get_array::<4>(&self.0, i) };
    jam::runtime::decode_u32(*p)
  }

  pub unsafe fn get_unchecked(&self, index: usize) -> u32 {
    let i = 4 * index;
    let p = unsafe { jam::runtime::unchecked::get_array::<4>(&self.0, i) };
    jam::runtime::decode_u32(*p)
  }
}

// struct Bar
//   a Foo
//   b U64
//   c Array[Foo]
//   d Array[U32]
// end

#[repr(transparent)]
pub struct Bar([u8]);

impl Bar {
  pub fn a(&self) -> &Foo {
    let i = 0;
    let p = unsafe { jam::runtime::unchecked::get_array::<8>(&self.0, i) };
    unsafe { core::mem::transmute::<&[u8; 8], &Foo>(p) }
  }

  pub fn b(&self) -> u64 {
    let i = 8;
    let p = unsafe { jam::runtime::unchecked::get_array::<8>(&self.0, i) };
    jam::runtime::decode_u64(*p)
  }

  pub fn c(&self) -> &ArrayFoo {
    let i = 16;
    let j = self.0.len();
    let p = unsafe { jam::runtime::unchecked::get_slice(&self.0, i, j) };
    unsafe { core::mem::transmute::<&[u8], &ArrayFoo>(p) }
  }
}

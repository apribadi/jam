use std::path::Path;
use std::sync::OnceLock;

static CARGO_WORKSPACE_DIR: OnceLock<&'static str> =
  OnceLock::new();

fn cargo_workspace_dir() -> &'static str {
  CARGO_WORKSPACE_DIR.get_or_init(|| {
    if let Ok(s) = std::env::var("CARGO_WORKSPACE_DIR") {
      return s.leak();
    }

    ""
  })
}

pub fn include(file: &str, line: u32) {
  let update = true;

  let path = Path::new(cargo_workspace_dir()).join(file);
  let file = std::fs::read(&path).unwrap();
  let file = std::str::from_utf8(&file).unwrap();

  let mut eol = 0;

  for (i, s) in file.split_inclusive('\n').enumerate() {
    eol += s.len();
    if i + 1 == line as usize { break; }
  }

  let (a, b) = file.split_at(eol);

  let c = compile();

  if b != c {
    if update {
      let file = format!("{}{}", a, c);
      std::fs::write(&path, file).unwrap();
    } else {
      panic!("b = \"{}\"\nc = \"{}\"", b, c)
    }
  }
}

#[derive(Clone, Copy)]
enum PrimitiveType {
  F32,
  F64,
  I8,
  I16,
  I32,
  I64,
  U8,
  U16,
  U32,
  U64,
  Bool,
}

impl PrimitiveType {
  fn stride(self) -> usize {
    match self {
      Self::F32 => 4,
      Self::F64 => 8,
      Self::I8 => 1,
      Self::I16 => 2,
      Self::I32 => 4,
      Self::I64 => 8,
      Self::U8 => 1,
      Self::U16 => 2,
      Self::U32 => 4,
      Self::U64 => 8,
      Self::Bool => 1,
    }
  }
}

enum ValueType {
  Primitive(PrimitiveType),
  Struct(),
}

impl ValueType {
  fn stride(self) -> usize {
    match self {
      Self::Primitive(ty) => ty.stride(),
      Self::Struct() => 0,
    }
  }
}

fn compile() -> String {
"
// START OF GENERATED CODE
".to_string()
}

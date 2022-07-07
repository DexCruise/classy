#[derive(Debug)]
enum InvalidFlagsError {
  InterfaceMustBeAbstract,

}

struct AccessFlagsBuilder {
  n: u16,
}

impl AccessFlagsBuilder {
  const PUBLIC: u16 = 0x0001;
  const FINAL: u16 = 0x0010;
  const SUPER: u16 = 0x0020;
  const INTERFACE: u16 = 0x0200;
  const ABSTRACT: u16 = 0x0400;
  const SYNTHETIC: u16 = 0x1000;
  const ANNOTATION: u16 = 0x2000;
  const ENUM: u16 = 0x4000;

  fn new() -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: 0 }
  }

  fn from(n: u16) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n }
  }

  fn is_set(&self, n: u16) -> bool {
    self.n | n == self.n
  }

  fn verify(self) -> Result<AccessFlagsBuilder, InvalidFlagsError> {
    use InvalidFlagsError::*;

    if self.is_set(AccessFlagsBuilder::INTERFACE)
        && !self.is_set(AccessFlagsBuilder::ABSTRACT) {
      Err(InterfaceMustBeAbstract)
    }



    Ok(self)
  }

  fn j_public(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::PUBLIC }
  }

  fn j_final(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::FINAL }
  }

  fn j_super(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::SUPER }
  }

  fn j_interface(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::INTERFACE }
  }

  fn j_abstract(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::ABSTRACT }
  }

  fn j_synthetic(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::SYNTHETIC }
  }

  fn j_annotation(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::ANNOTATION }
  }

  fn j_enum(self) -> AccessFlagsBuilder {
    AccessFlagsBuilder { n: self.n | AccessFlagsBuilder::ENUM }
  }
}

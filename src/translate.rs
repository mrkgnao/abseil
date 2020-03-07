use crate::patch::*;
use crate::traits::*;

pub trait Translate: AbGroup + Patch {
  fn lift(self) -> Delta<Self>;
}

use crate::patch::*;

pub enum Fun<A: Patch, B: Patch, C>
{
  Primitive {
    fun: Box<dyn Fn(A) -> (B, C)>,
    diff_fun: Box<dyn Fn(A, Delta<A>, C) -> (Delta<B>, C)>,
  },
}

pub enum DiffFun<A: Patch, B: Patch, C>
{
  Primitive {
    diff_fun: Box<dyn Fn(A, Delta<A>, C) -> (Delta<B>, C)>,
  },
}

impl<A: Patch, B: Patch, C> Fun<A, B, C> {
    fn apply(self, a: A) -> (B, C) {
        match self {
            Fun::Primitive { fun, .. } => fun(a),
        }
    }
}

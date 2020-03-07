#![allow(dead_code)]
pub mod cache;
pub mod multiset;
pub mod patch;
pub mod traits;
pub mod translate;
pub mod functions;

use crate::cache::*;
use crate::multiset::*;
use crate::patch::*;
use crate::traits::*;
use crate::translate::*;

use std::collections::HashMap;
use std::hash::Hash;

fn main() {
  let b = Multiset::new(
    [(Sum::new(1), Sum::new(2)), (Sum::new(11), Sum::new(-1))]
      .iter()
      .cloned(),
  );
  println!("{:?}", b.fold_group());
}

pub struct Plus;
pub struct PlusCache;
impl HasCache for Plus {
  type Cache = PlusCache;
}

fn cplus(x: i32, y: i32) -> Caching<Plus, i32> {
  Caching {
    data: x + y,
    cache: PlusCache,
  }
}

fn dplus(
  (_x, _y): (i32, i32),
  (dx, dy): (i32, i32),
  cache: PlusCache,
) -> Caching<Plus, Delta<i32>> {
  Caching {
    data: Delta(dx + dy),
    cache,
  }
}

pub struct Div;
pub struct DivCache;
impl HasCache for Div {
  type Cache = DivCache;
}

fn cdiv(x: i32, y: i32) -> Caching<Div, i32> {
  Caching {
    data: x / y,
    cache: DivCache,
  }
}

fn ddiv(
  (x, y): (i32, i32),
  (dx, dy): (i32, i32),
  cache: DivCache,
) -> Caching<Div, Delta<i32>> {
  Caching {
    data: Delta((x + dx) / (y + dy) - x / y),
    cache,
  }
}

fn multiset_sum(b: Multiset<Sum<i32>>) -> Sum<i32> {
  b.fold_group()
}

// data Fun a b c where
//   Primitive ::
//     !(a -> (b, c)) -> !(a -> Dt a -> c -> (Dt b, c)) -> Fun a b c
//   Closure :: (NilTestable e, NilChangeStruct e, NFData e) =>
//     !(Fun (e, a) b c) -> !e -> Fun a b c
// data DFun a b c where
//   DPrimitive ::
//     !(a -> Dt a -> c -> (Dt b, c)) -> DFun a b c
//   DClosure :: (NilTestable e, NilChangeStruct e, NFData e) =>
//     !(Fun (e, a) b c) -> !(DFun (e, a) b c) -> !e -> !(Dt e) ->
// DFun a b c


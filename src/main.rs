#![allow(dead_code)]
pub mod multiset;
pub mod patch;
pub mod traits;
pub mod translate;

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

pub trait HasCache {
  type Cache;
}

pub struct CachedValue<Op: HasCache, T> {
  value: T,
  cache: <Op as HasCache>::Cache,
}

pub struct CachedDelta<Op: HasCache, T: Patch> {
  delta: Delta<T>,
  cache: <Op as HasCache>::Cache,
}

pub struct Plus;
pub struct PlusCache;
impl HasCache for Plus {
  type Cache = PlusCache;
}

fn cplus(x: i32, y: i32) -> CachedValue<Plus, i32> {
  CachedValue {
    value: x + y,
    cache: PlusCache,
  }
}

fn dplus(
  (_x, _y): (i32, i32),
  (dx, dy): (i32, i32),
  cache: PlusCache,
) -> CachedDelta<Plus, i32> {
  CachedDelta {
    delta: Delta(dx + dy),
    cache,
  }
}

pub struct Div;
pub struct DivCache;
impl HasCache for Div {
  type Cache = DivCache;
}

fn cdiv(x: i32, y: i32) -> CachedValue<Div, i32> {
  CachedValue {
    value: x / y,
    cache: DivCache,
  }
}

fn ddiv(
  (x, y): (i32, i32),
  (dx, dy): (i32, i32),
  cache: DivCache,
) -> CachedDelta<Div, i32> {
  CachedDelta {
    delta: Delta((x + dx) / (y + dy) - x / y),
    cache,
  }
}

/*
sum =
  (\a0 ->
     in let x0 = foldGroup a0
        in let x1 = getSum x0
           in x1)
*/

fn sum_multiset(b: Multiset<Sum<i32>>) -> Sum<i32> {
    b.fold_group()
}

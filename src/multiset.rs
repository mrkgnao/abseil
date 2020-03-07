use crate::cache::*;
use crate::traits::*;
use crate::patch::*;
use crate::functions::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{FromIterator, Iterator};
use std::ops::{Add, AddAssign, Neg};

#[derive(Debug, Clone)]
pub struct Multiset<T> {
  inner: HashMap<T, Sum<i32>>,
}

impl<T: Hash + Eq + Clone> Add for Multiset<T> {
  type Output = Multiset<T>;

  fn add(self, other: Multiset<T>) -> Multiset<T> {
    Multiset::new(self.inner.into_iter().chain(other.inner))
  }
}

impl<T: Hash + Eq + Clone> AddAssign for Multiset<T> {
  fn add_assign(&mut self, other: Multiset<T>) {
    self.inner.extend(other.inner);
  }
}

impl<T: Hash + Eq + Clone> Neg for Multiset<T> {
  type Output = Multiset<T>;
  fn neg(self) -> Multiset<T> {
    Multiset {
      inner: self.inner.into_iter().map(|(k, v)| (k, -v)).collect(),
    }
  }
}

impl<T: Hash + Eq + Clone> Semigroup for Multiset<T> {}

impl<T: Hash + Eq + Clone> Monoid for Multiset<T> {
  fn nil() -> Self {
    Multiset {
      inner: HashMap::new(),
    }
  }
}

impl<T: Hash + Eq + Clone> AbGroup for Multiset<T> {}

impl<T: Hash + Eq + Clone> Multiset<T> {
  pub fn singleton(t: T) -> Multiset<T> {
    let mut h = HashMap::new();
    h.insert(t, Sum::new(1));
    Multiset::new(h.into_iter())
  }
}

pub struct FoldGroupCache;
pub struct MapCache<A, B, C>(HashMap<A, (B, C)>);
pub struct FoldMapGroupCache;
pub struct SingletonCache;

impl<T> Multiset<T> {
  pub fn new<A: Iterator>(data: A) -> Multiset<T>
  where
    HashMap<T, Sum<i32>>: FromIterator<A::Item>,
  {
    Multiset {
      inner: data.collect(),
    }
  }

  // pub fn map(
  pub fn fold_group(self) -> T
  where
    T: AbGroup,
  {
    self
      .inner
      .into_iter()
      .fold(T::nil(), |acc, (k, v)| acc + k.scale(v.get_sum()))
  }

  pub fn map<A, F>(self, f: F) -> Multiset<A>
  where
    F: Fn(T) -> A,
    A: Hash + Eq,
  {
    Multiset::new(self.inner.into_iter().filter_map(|(k, v)| {
      if v.get_sum() == 0 {
        None
      } else {
        Some((f(k), v))
      }
    }))
  }

  pub fn fold_map_group<A, F>(self, f: F) -> A
  where
    F: Fn(T) -> A,
    A: Hash + Eq + AbGroup,
    T: AbGroup,
  {
    self.map(f).fold_group()
  }

  pub fn caching_fold_group(self) -> (T, FoldGroupCache)
  where
    T: AbGroup,
  {
      (self.fold_group(), FoldGroupCache)
  }

  // pub fn caching_map<A, B, C, F>(self, f: Fun<A, B, C>) -> (Multiset<A>, MapCache<A, B, C>)
  // where
  //   A: Hash + Eq + Patch,
  //   B: Patch,
  // {
    // let (r, _) = 
  // }
}

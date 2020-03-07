use crate::traits::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{FromIterator, Iterator};
use std::ops::{Add, AddAssign, Neg};

#[derive(Debug, Clone)]
pub struct Multiset<T> {
  inner: HashMap<T, Sum<i32>>,
}

impl<T> Multiset<T> {
  pub fn new<A: Iterator>(data: A) -> Multiset<T>
  where
    HashMap<T, Sum<i32>>: FromIterator<A::Item>,
  {
    Multiset {
      inner: data.collect(),
    }
  }
}

impl<T: Hash + Eq + Clone> Add for Multiset<T> {
  type Output = Multiset<T>;

  fn add(self, other: Multiset<T>) -> Multiset<T> {
    Multiset {
      inner: self.inner.into_iter().chain(other.inner).collect(),
    }
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
}

impl<T> Multiset<T> {
  pub fn fold_map_group<A, F>(self, f: F) -> A
  where
    F: Fn(T) -> A,
    A: Hash + Eq + AbGroup,
  {
    self
      .inner
      .into_iter()
      .filter_map(|(k, v)| {
        if v.get_sum() == 0 {
          None
        } else {
          Some((f(k), v))
        }
      })
      .fold(A::nil(), |acc, (k, v)| acc + k.scale(v.get_sum()))
  }
}

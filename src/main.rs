#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Neg};

fn main() {
  let b = Bag {
    inner: [(1, 2), (11, -1)].iter().cloned().collect(),
  };
  println!("{}", b.fold_group());
}

pub trait Semigroup:
  Add<Self, Output = Self> + AddAssign<Self> + Sized
{
  fn append(self, other: Self) -> Self {
    self + other
  }

  fn append_mut(&mut self, other: Self) {
    *self += other;
  }
}
pub trait Monoid: Semigroup + Clone {
  fn nil() -> Self;
  fn nonneg_scale(self, mut exponent: u32) -> Self {
      let mut result = Self::nil();
      while exponent > 0 {
          result += self.clone();
          exponent -= 1;
      }
      result
  }
}
/// TODO negate_mut
pub trait AbGroup: Monoid + Neg<Output = Self> {
  fn negate(self) -> Self {
    -self
  }

  fn scale(self, exponent: i32) -> Self {
    use std::cmp::Ordering;
    let abs_exp = exponent.abs() as u32;
    match exponent.cmp(&0) {
      Ordering::Less => -self.nonneg_scale(abs_exp),
      Ordering::Equal | Ordering::Greater => {
        self.nonneg_scale(abs_exp)
      }
    }
  }
}

impl Semigroup for i32 {}
impl Monoid for i32 {
  fn nil() -> i32 {
    0
  }

  fn nonneg_scale(self, exponent: u32) -> Self {
      self * (exponent as i32)
  }
}
impl AbGroup for i32 {}

pub struct Delta<T: ?Sized + Patch>(pub <T as Patch>::Delta);

pub trait Patch {
  type Delta;
  fn patch(&self, delta: Delta<Self>) -> Self;
  fn patch_mut(&mut self, delta: Delta<Self>)
  where
    Self: Sized,
  {
    *self = self.patch(delta);
  }
}

impl Patch for () {
  type Delta = ();
  fn patch(&self, _delta: Delta<()>) -> () {
    ()
  }
}

impl<A: Patch, B: Patch> Patch for (A, B) {
  type Delta = (<A as Patch>::Delta, <B as Patch>::Delta);
  fn patch(&self, delta: Delta<(A, B)>) -> (A, B) {
    let (a, b) = self;
    let Delta((da, db)) = delta;
    (a.patch(Delta(da)), b.patch(Delta(db)))
  }
}

impl Patch for i32 {
  type Delta = i32;
  fn patch(&self, delta: Delta<i32>) -> i32 {
    self + delta.0
  }
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

#[derive(Debug, Clone)]
pub struct Bag<T> {
  inner: HashMap<T, i32>,
}

impl<T: Hash + Eq + Clone> Add for Bag<T> {
  type Output = Bag<T>;

  fn add(self, other: Bag<T>) -> Bag<T> {
    Bag {
      inner: self.inner.into_iter().chain(other.inner).collect(),
    }
  }
}

impl<T: Hash + Eq + Clone> AddAssign for Bag<T> {
  fn add_assign(&mut self, other: Bag<T>) {
    self.inner.extend(other.inner);
  }
}

impl<T: Hash + Eq + Clone> Neg for Bag<T> {
  type Output = Bag<T>;
  fn neg(self) -> Bag<T> {
    Bag {
      inner: self.inner.into_iter().map(|(k, v)| (k, -v)).collect(),
    }
  }
}

impl<T: Hash + Eq + Clone> Semigroup for Bag<T> {}

impl<T: Hash + Eq + Clone> Monoid for Bag<T> {
  fn nil() -> Self {
    Bag {
      inner: HashMap::new(),
    }
  }
}

impl<T: Hash + Eq + Clone> AbGroup for Bag<T> {}

impl<T: Hash + Eq + Clone> Bag<T> {
  pub fn fold_group(self) -> T
  where
    T: AbGroup,
  {
    self
      .inner
      .into_iter()
      .fold(T::nil(), |acc, (k, v)| acc + k.scale(v))
  }
}

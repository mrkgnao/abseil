use std::ops::{Add, AddAssign, Neg};
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct Sum<T>(T);

impl<T> Sum<T> {
  pub fn new(t: T) -> Sum<T> {
    Sum(t)
  }
  pub fn get_sum(self) -> T {
    self.0
  }
}

impl<A: Add<Output = A>> Add for Sum<A> {
  type Output = Sum<A>;

  fn add(self, other: Sum<A>) -> Sum<A> {
    Sum::new(self.get_sum().add(other.get_sum()))
  }
}

impl<A: AddAssign> AddAssign for Sum<A> {
  fn add_assign(&mut self, other: Sum<A>) {
    self.0.add_assign(other.0);
  }
}

impl<A: Neg<Output = A>> Neg for Sum<A> {
  type Output = Sum<A>;
  fn neg(self) -> Sum<A> {
    Sum::new(self.0.neg())
  }
}

impl Semigroup for Sum<i32> {}
impl Monoid for Sum<i32> {
  fn nil() -> Sum<i32> {
    Sum::new(0)
  }

  fn nonneg_scale(self, exponent: u32) -> Self {
    Sum::new(self.get_sum() * (exponent as i32))
  }
}
impl AbGroup for Sum<i32> {}

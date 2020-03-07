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

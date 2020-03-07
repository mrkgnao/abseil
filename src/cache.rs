pub trait HasCache {
  type Cache;
}

pub struct Caching<Op: HasCache, T> {
  pub data: T,
  pub cache: <Op as HasCache>::Cache,
}

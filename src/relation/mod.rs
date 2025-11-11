use std::collections::HashSet;
use std::hash::Hash;

pub type Relation<A, B> = HashSet<(A, B)>;

pub trait Closure<A> {
  fn reflexive(&self) -> Relation<A, A>;
  fn symmetric(&self) -> Relation<A, A>;
  fn transitive(&self) -> Relation<A, A>;
}

impl<A : Clone + Copy + Eq + Hash> Closure<A> for Relation<A, A> {
  fn reflexive(&self) -> Relation<A, A> {
    let mut new = self.clone();
    for (a, _) in self {
      let aa = a.clone();
      new.insert((aa, aa));
    }
    new
  }
  fn symmetric(&self) -> Relation<A, A> {
    let mut new = self.clone();
    for (a, b) in self {
      new.insert((b.clone(), a.clone()));
    }
    new
  }
  fn transitive(&self) -> Relation<A, A> {
    let mut new = self.clone();
    for (a, b) in self {
      for (aa, bb) in self {
        if b == aa {
          new.insert((a.clone(), bb.clone()));
        } else {
          ()
        }
      }
    }
    new
  }
}
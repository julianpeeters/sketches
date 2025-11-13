use nonempty::NonEmpty;
use std::collections::HashSet;
use std::hash::Hash;

pub type Relation<A, B> = HashSet<(A, B)>;

pub type EquivalenceClass<A> = NonEmpty<Relation<A, A>>;

pub trait Closure<A> {
  fn equivalence(&self) -> Relation<A, A>;
  fn reflexivity(&self) -> Relation<A, A>;
  fn symmetry(&self) -> Relation<A, A>;
  fn transitivity(&self) -> Relation<A, A>;
}

pub trait Equivalence<A> {
  fn get_connections(map: &Relation<A, A>) -> Relation<A, A>;
  fn equivalents_by<F>(&self, f: F) -> Vec<EquivalenceClass<A>>
  where F: Fn(&Relation<A, A>) -> Relation<A, A>;
}

impl<A : Clone + Copy + Eq + Hash> Closure<A> for Relation<A, A> {
  fn equivalence(&self) -> Relation<A, A> {
    self.reflexivity()
        .symmetry()
        .transitivity()
        .symmetry()
  }
  fn reflexivity(&self) -> Relation<A, A> {
    let mut new = self.clone();
    for (a, _) in self {
      let aa = a.clone();
      new.insert((aa, aa));
    }
    new
  }
  fn symmetry(&self) -> Relation<A, A> {
    let mut new = self.clone();
    for (a, b) in self {
      new.insert((b.clone(), a.clone()));
    }
    new
  }
  fn transitivity(&self) -> Relation<A, A> {
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
    if self == &new { new }
    else { new.transitivity() }
  }
}

impl<A : Copy + Eq + Hash> Equivalence<A> for NonEmpty<Relation<A, A>> {

  fn get_connections(map: &Relation<A, A>) -> Relation<A, A> {
      map.iter().fold(HashSet::new(), |mut acc, e| {
        if e.0 == e.1 {
          acc
        } else {
          acc.insert((e.0, e.1));
          acc
        }
      })
    }

  fn equivalents_by<F>(&self, f: F) -> Vec<EquivalenceClass<A>>
  where F: Fn(&Relation<A, A>) -> Relation<A, A> {
    let mut acc: Vec<NonEmpty<Relation<A, A>>> = Vec::new();
    let query_connections: Relation<A, A> = f(&Self::get_connections(self.first()));
    let pp: (Vec<&Relation<A, A>>, Vec<&Relation<A, A>>) = 
      self.iter().partition(|&candidate| {
        query_connections == f(&Self::get_connections(candidate))
      });
    let _ = NonEmpty::from_vec(pp.0.into_iter().cloned().collect()).map_or((), |x| acc.push(x));
    let _ = NonEmpty::from_vec(pp.1.into_iter().cloned().collect()).map_or((), |x| acc.append(&mut x.equivalents_by(f)));
    acc
  }
  
}
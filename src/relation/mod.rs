use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

pub type Relation<A, B> = HashSet<(A, B)>;

pub type EquivalenceClass<A> = Vec<Relation<A, A>>;

pub trait Closure<A> {
  fn equivalence(&self) -> Relation<A, A>;
  fn reflexivity(&self) -> Relation<A, A>;
  fn symmetry(&self) -> Relation<A, A>;
  fn transitivity(&self) -> Relation<A, A>;
}

pub trait Equivalence<A> {
  fn equivalents_by<F>(&self, f: F) -> Vec<EquivalenceClass<A>>
  where F: Fn(&Relation<A, A>) -> Relation<A, A>;
}

impl<A : Clone + Copy + Eq + Hash> Closure<A> for Relation<A, A> {
  fn equivalence(&self) -> Relation<A, A> {
    self.reflexivity()
        .symmetry()
        .transitivity()
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

impl<A : Copy + Eq + Hash> Equivalence<A> for Vec<Relation<A, A>> {

  fn equivalents_by<F>(&self, f: F) -> Vec<EquivalenceClass<A>>
  where F: Fn(&Relation<A, A>) -> Relation<A, A> {

    fn test<F, A : Copy + Eq + Hash>(f: F, it: &EquivalenceClass<A>, query: &Relation<A, A>, mut acc: Vec<EquivalenceClass<A>>) -> Vec<EquivalenceClass<A>>
    where F : Fn(&Relation<A, A>) -> Relation<A, A> {

      fn get_connections<A : Copy + Eq + Hash>(map: &Relation<A, A>) -> Relation<A, A> {
        map.iter().fold(HashSet::new(), |mut acc, e| {
          if e.0 == e.1 {
            acc
          } else {
            acc.insert((e.0, e.1));
            acc
          }
        })
      }
      let pp: (Vec<&Relation<A, A>>, Vec<&Relation<A, A>>) = 
        it.iter().partition(|&candidate| {
          f(&get_connections(query)) == f(&get_connections(candidate))
        });
      acc.push(pp.0.iter().map(|x| (*x).clone()).collect());
      acc.append(&mut pp.1.iter().map(|x| (*x).clone()).collect_vec().equivalents_by(f));
      acc
    }

    match self.first() {
      Some(query) => test(f, self, query, Vec::new()),
      None => Vec::new(),
    }

  }
  
}
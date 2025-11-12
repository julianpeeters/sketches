use crate::function::Tetration;
use crate::set::{Zero, One, Two, Three, Four, Five};
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

pub type Relation<A, B> = HashSet<(A, B)>;

pub type Part<A> = Vec<Relation<A, A>>;

pub trait Closure<A> {
  fn equivalence(&self) -> Relation<A, A>;
  fn reflexivity(&self) -> Relation<A, A>;
  fn symmetry(&self) -> Relation<A, A>;
  fn transitivity(&self) -> Relation<A, A>;
}

pub trait Equivalence<A> {
  fn equivalents_by<F>(&self, f: F) -> Vec<Part<A>>
  where F: Fn(&Relation<A, A>) -> Relation<A, A>;
}

pub trait Partition<A> {
  fn partition(&self) -> Vec<Part<&A>>;
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

  fn equivalents_by<F>(&self, f: F) -> Vec<Part<A>>
  where F: Fn(&Relation<A, A>) -> Relation<A, A> {

    fn test<F, A : Copy + Eq + Hash>(f: F, it: &Part<A>, query: &Relation<A, A>, mut acc: Vec<Part<A>>) -> Vec<Part<A>>
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

impl Partition<Zero> for [&Zero; 0]{
  fn partition(&self) -> Vec<Part<&Zero>> {
    vec!()
  }
}

impl Partition<One> for [&One; 1]{
  fn partition(&self) -> Vec<Part<&One>> {
    self.tetrate().equivalents_by(|x| x.equivalence())
  }
}

impl Partition<Two> for [&Two; 2]{
  fn partition(&self) -> Vec<Part<&Two>> {
    self.tetrate().equivalents_by(|x| x.equivalence())
  }
}

impl Partition<Three> for [&Three; 3]{
  fn partition(&self) -> Vec<Part<&Three>> {
    self.tetrate().equivalents_by(|x| x.equivalence())
  }
}

impl Partition<Four> for [&Four; 4]{
  fn partition(&self) -> Vec<Part<&Four>> {
    self.tetrate().equivalents_by(|x| x.equivalence())
  }
}

impl Partition<Five> for [&Five; 5]{
  fn partition(&self) -> Vec<Part<&Five>> {
    self.tetrate().equivalents_by(|x| x.equivalence())
  }
}
use crate::{
  function::Tetration,
  relation::{Closure, Equivalence, EquivalenceClass},
  set::{Five, Four, One, Three, Two, Zero}
};

pub type Part<A> = EquivalenceClass<A>;

pub trait Partition<A> {
  fn partition(&self) -> Vec<Part<&A>>;
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
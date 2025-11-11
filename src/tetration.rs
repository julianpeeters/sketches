use crate::set::{Zero, One, Two, Three, Four, Five};
use itertools::Itertools;
use std::collections::HashMap;

pub trait Tetration<A> {
  fn tetrate(&self) -> Vec<HashMap<&A, &A>>;
}

impl Tetration<Zero> for [&Zero; 0] {
  fn tetrate(&self) -> Vec<HashMap<&Zero, &Zero>> {
    self.iter()
        .map(|_| {
          let functions = HashMap::new();
          functions
        })
        .collect_vec()
  }
}

impl Tetration<One> for [&One; 1] {
  fn tetrate(&self) -> Vec<HashMap<&One, &One>> {
    self.iter()
        .map(|p| {
          let mut functions = HashMap::new();
          functions.insert(&One::_1, *p);
          functions
        })
        .collect_vec()
  }
}

impl Tetration<Two> for [&Two; 2] {
  fn tetrate(&self) -> Vec<HashMap<&Two, &Two>> {
    self.iter().cartesian_product(self)
        .map(|p| {
          let mut functions = HashMap::new();
          functions.insert(&Two::_1, *p.0);
          functions.insert(&Two::_2, *p.1);
          functions
        })
        .collect_vec()
  }
}

impl Tetration<Three> for [&Three; 3] {
  fn tetrate(&self) -> Vec<HashMap<&Three, &Three>> {
    self.iter().cartesian_product(self).cartesian_product(self)
        .map(|p| {
          let mut functions = HashMap::new();
          functions.insert(&Three::_1, *p.0.0);
          functions.insert(&Three::_2, *p.0.1);
          functions.insert(&Three::_3, *p.1);
          functions
        })
        .collect_vec()
  }
}

impl Tetration<Four> for [&Four; 4] {
  fn tetrate(&self) -> Vec<HashMap<&Four, &Four>> {
    self.iter().cartesian_product(self).cartesian_product(self).cartesian_product(self)
        .map(|p| {
          let mut functions = HashMap::new();
          functions.insert(&Four::_1, *p.0.0.0);
          functions.insert(&Four::_2, *p.0.0.1);
          functions.insert(&Four::_3, *p.0.1);
          functions.insert(&Four::_4, *p.1);
          functions
        })
        .collect_vec()
  }
}

impl Tetration<Five> for [&Five; 5] {
  fn tetrate(&self) -> Vec<HashMap<&Five, &Five>> {
    self.iter().cartesian_product(self).cartesian_product(self).cartesian_product(self).cartesian_product(self)
        .map(|p| {
          let mut functions = HashMap::new();
          functions.insert(&Five::_1, *p.0.0.0.0);
          functions.insert(&Five::_2, *p.0.0.0.1);
          functions.insert(&Five::_3, *p.0.0.1);
          functions.insert(&Five::_4, *p.0.1);
          functions.insert(&Five::_5, *p.1);
          functions
        })
        .collect_vec()
  }
}
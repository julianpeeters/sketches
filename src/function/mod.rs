use crate::set::{Five, Four, One, Three, Two, Zero};
use itertools::Itertools;
use nonempty::{nonempty, NonEmpty};
use std::collections::HashSet;

type Function<A, B> = HashSet<(A, B)>;

pub trait Tetration<A> {
  fn tetrate(&self) -> NonEmpty<Function<&A, &A>>;
}

impl Tetration<Zero> for [&Zero; 0] {
  fn tetrate(&self) -> NonEmpty<Function<&Zero, &Zero>> {
    panic!("The tetration of 0 is undefined");
  }
}

impl Tetration<One> for [&One; 1] {
  fn tetrate(&self) -> NonEmpty<Function<&One, &One>> {
    NonEmpty::from_vec(self.to_vec())
      .map_or(nonempty![&One::_1], |x| x)
      .map(|p| {
        let mut functions = HashSet::new();
        functions.insert((&One::_1, p));
        functions
      })
  }
}

impl Tetration<Two> for [&Two; 2] {
  fn tetrate(&self) -> NonEmpty<Function<&Two, &Two>> {
    NonEmpty::from_vec(self.iter().cartesian_product(self).collect())
      .map_or(nonempty![(&&Two::_1, &&Two::_2)], |x| x)
      .map(|p| {
        let mut functions = HashSet::new();
        functions.insert((&Two::_1, *p.0));
        functions.insert((&Two::_2, *p.1));
        functions
      })
  }
}


impl Tetration<Three> for [&Three; 3] {
  fn tetrate(&self) -> NonEmpty<Function<&Three, &Three>> {
    NonEmpty::from_vec(self.iter().cartesian_product(self).cartesian_product(self).collect())
      .map_or(nonempty![((&&Three::_1, &&Three::_2), &&Three::_3)], |x| x)
      .map(|p| {
        let mut functions = HashSet::new();
        functions.insert((&Three::_1, *p.0.0));
        functions.insert((&Three::_2, *p.0.1));
        functions.insert((&Three::_3, *p.1));
        functions
      })
  }
}

impl Tetration<Four> for [&Four; 4] {
  fn tetrate(&self) -> NonEmpty<Function<&Four, &Four>> {
    NonEmpty::from_vec(self.iter().cartesian_product(self).cartesian_product(self).cartesian_product(self).collect())
      .map_or(nonempty![(((&&Four::_1, &&Four::_2), &&Four::_3), &&Four::_4)], |x| x)
      .map(|p| {
        let mut functions = HashSet::new();
        functions.insert((&Four::_1, *p.0.0.0));
        functions.insert((&Four::_2, *p.0.0.1));
        functions.insert((&Four::_3, *p.0.1));
        functions.insert((&Four::_4, *p.1));
        functions
      })
  }
}

impl Tetration<Five> for [&Five; 5] {
  fn tetrate(&self) -> NonEmpty<Function<&Five, &Five>> {
    NonEmpty::from_vec(self.iter().cartesian_product(self).cartesian_product(self).cartesian_product(self).cartesian_product(self).collect())
      .map_or(nonempty![((((&&Five::_1, &&Five::_2), &&Five::_3), &&Five::_4), &&Five::_5)], |x| x)
      .map(|p| {
        let mut functions = HashSet::new();
        functions.insert((&Five::_1, *p.0.0.0.0));
        functions.insert((&Five::_2, *p.0.0.0.1));
        functions.insert((&Five::_3, *p.0.0.1));
        functions.insert((&Five::_4, *p.0.1));
        functions.insert((&Five::_5, *p.1));
        functions
      })
  }
}
// fn tetration1() -> Vec<Map<Two, Two>> {
//     Two::VALUES.iter().cartesian_product(Two::VALUES.iter())
//     .map(|p| vec!((Two::_1, *p.0), (Two::_2, *p.1)))
//     .collect_vec()
//   }

use crate::set::Two;
use itertools::Itertools;
use std::collections::HashMap;


pub trait Tetration<A> {
  fn tetrate(&self) -> Vec<HashMap<&A, &A>>;
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
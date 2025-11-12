use std::collections::HashSet;

use sketches::{
  function::Tetration,
  relation::{Closure, Relation},
  set::{Five, Four, One, Three, Two, Zero, partition::Partition},
};

#[test]
fn test_public_api() {

  assert_eq!(Two::VALUES, [&Two::_1, &Two::_2]);

  let mut a: HashSet<(&Two, &Two)> = HashSet::new();
  let mut b: HashSet<(&Two, &Two)> = HashSet::new();
  let mut c: HashSet<(&Two, &Two)> = HashSet::new();
  let mut d: HashSet<(&Two, &Two)> = HashSet::new();
  a.insert((&Two::_1, &Two::_1)); a.insert((&Two::_2, &Two::_1));
  b.insert((&Two::_1, &Two::_1)); b.insert((&Two::_2, &Two::_2));
  c.insert((&Two::_1, &Two::_2)); c.insert((&Two::_2, &Two::_1));
  d.insert((&Two::_1, &Two::_2)); d.insert((&Two::_2, &Two::_2));
  assert_eq!(Two::VALUES.tetrate(), vec!(a, b, c, d));

  let mut e: Relation<&Two, &Two> = Relation::new();
  let mut f: Relation<&Two, &Two> = Relation::new();
  e.insert((&Two::_1, &Two::_2));
  f.insert((&Two::_1, &Two::_2));
  f.insert((&Two::_1, &Two::_1));
  assert_eq!(e.reflexivity(), f);
  
  let mut g: HashSet<(&One, &One)> = HashSet::new();
  let mut h: HashSet<(&Two, &Two)> = HashSet::new();
  let mut i: HashSet<(&Two, &Two)> = HashSet::new();
  let mut j: HashSet<(&Two, &Two)> = HashSet::new();
  let mut k: HashSet<(&Two, &Two)> = HashSet::new();
  g.insert((&One::_1, &One::_1));
  h.insert((&Two::_1, &Two::_1)); h.insert((&Two::_2, &Two::_1));
  i.insert((&Two::_1, &Two::_1)); i.insert((&Two::_2, &Two::_2));
  j.insert((&Two::_1, &Two::_2)); j.insert((&Two::_2, &Two::_1));
  k.insert((&Two::_1, &Two::_2)); k.insert((&Two::_2, &Two::_2));
  assert_eq!(Zero::VALUES.partition(), vec!() as Vec<Vec<HashSet<(&Zero, &Zero)>>>);
  assert_eq!(One::VALUES.partition(), vec!(vec!(g)));
  assert_eq!(Two::VALUES.partition(), vec!(vec!(h, j, k), vec!(i)));
  assert_eq!(Three::VALUES.partition().len(), 5);
  assert_eq!(Four::VALUES.partition().len(), 15);
  assert_eq!(Five::VALUES.partition().len(), 52);

}
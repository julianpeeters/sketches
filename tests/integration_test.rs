use nonempty::{NonEmpty, nonempty};
use std::collections::HashSet;
use sketches::{
  function::Tetration,
  order::Join,
  relation::{Closure, Relation},
  set::{Five, Four, One, Three, Two, Zero, partition::Partition},
};

#[test]
fn test_public_api() {

  assert_eq!(Two::SET, [&Two::_1, &Two::_2]);

  let mut a: HashSet<(&Two, &Two)> = HashSet::new();
  let mut b: HashSet<(&Two, &Two)> = HashSet::new();
  let mut c: HashSet<(&Two, &Two)> = HashSet::new();
  let mut d: HashSet<(&Two, &Two)> = HashSet::new();
  a.insert((&Two::_1, &Two::_1)); a.insert((&Two::_2, &Two::_1));
  b.insert((&Two::_1, &Two::_1)); b.insert((&Two::_2, &Two::_2));
  c.insert((&Two::_1, &Two::_2)); c.insert((&Two::_2, &Two::_1));
  d.insert((&Two::_1, &Two::_2)); d.insert((&Two::_2, &Two::_2));
  assert_eq!(Two::SET.tetrate(), nonempty![a, b, c, d]);

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
  assert_eq!(Zero::SET.partition(), vec!() as Vec<NonEmpty<HashSet<(&Zero, &Zero)>>>);
  assert_eq!(One::SET.partition(), vec!(nonempty![g]));
  assert_eq!(Two::SET.partition(), vec!(nonempty!(h, j, k), nonempty!(i)));
  assert_eq!(Three::SET.partition().len(), 5);
  assert_eq!(Four::SET.partition().len(), 15);
  assert_eq!(Five::SET.partition().len(), 52);

  let mut l: HashSet<(&Two, &Two)> = HashSet::new();
  let mut m: HashSet<(&Two, &Two)> = HashSet::new();
  let mut n: HashSet<(&Two, &Two)> = HashSet::new();
  let mut o: HashSet<(&Two, &Two)> = HashSet::new();
  l.insert((&Two::_1, &Two::_1)); l.insert((&Two::_2, &Two::_1));
  m.insert((&Two::_1, &Two::_1)); m.insert((&Two::_2, &Two::_2));
  n.insert((&Two::_1, &Two::_2)); n.insert((&Two::_2, &Two::_1));
  o.insert((&Two::_1, &Two::_2)); o.insert((&Two::_2, &Two::_2));
  assert_eq!(nonempty!(l.clone(), n.clone(), o.clone()).join(&nonempty!(m)), nonempty!(l, n, o));

}
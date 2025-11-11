use std::collections::HashSet;

use sketches::{
  function::Tetration,
  relation::{Closure, Relation},
  set::Two,
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
  assert_eq!(e.reflexive(), f);

}
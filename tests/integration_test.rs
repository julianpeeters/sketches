use std::collections::HashMap;

use sketches::{
  set::Two, tetration::Tetration
};

#[test]
fn test_public_api() {

  assert_eq!(Two::VALUES, [&Two::_1, &Two::_2]);

  let mut a: HashMap<&Two, &Two> = HashMap::new();
  let mut b: HashMap<&Two, &Two> = HashMap::new();
  let mut c: HashMap<&Two, &Two> = HashMap::new();
  let mut d: HashMap<&Two, &Two> = HashMap::new();
  a.insert(&Two::_1, &Two::_1);
  a.insert(&Two::_2, &Two::_1);
  b.insert(&Two::_2, &Two::_2);
  b.insert(&Two::_1, &Two::_1);
  c.insert(&Two::_2, &Two::_1);
  c.insert(&Two::_1, &Two::_2);
  d.insert(&Two::_2, &Two::_2);
  d.insert(&Two::_1, &Two::_2);
  assert_eq!(Two::VALUES.tetrate(), vec!(a, b, c, d));

}
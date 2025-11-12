pub mod partition;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Zero {}
impl Zero {
  pub const VALUES: [&Zero; 0] = [];
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum One {
  _1,
}
impl One {
  pub const VALUES: [&One; 1] = [&One::_1];
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Two {
  _1,
  _2,
}
impl Two {
  pub const VALUES: [&Two; 2] = [&Two::_1, &Two::_2,];
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Three {
  _1,
  _2,
  _3,
}
impl Three {
  pub const VALUES: [&Three; 3] = [&Three::_1, &Three::_2, &Three::_3,];
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Four {
  _1,
  _2,
  _3,
  _4,
}
impl Four {
  pub const VALUES: [&Four; 4] = [&Four::_1, &Four::_2, &Four::_3, &Four::_4,];
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Five {
  _1,
  _2,
  _3,
  _4,
  _5,
}
impl Five {
  pub const VALUES: [&Five; 5] = [&Five::_1, &Five::_2, &Five::_3, &Five::_4, &Five::_5,];
}
enum Zero {}
impl Zero {
  const VALUES: [&Zero; 0] = [];
}

enum One {
  _1,
}
impl One {
  const VALUES: [&One; 1] = [&One::_1];
}

enum Two {
  _1,
  _2,
}
impl Two {
  const VALUES: [&Two; 2] = [&Two::_1, &Two::_2,];
}

enum Three {
  _1,
  _2,
  _3,
}
impl Three {
  const VALUES: [&Three; 3] = [&Three::_1, &Three::_2, &Three::_3,];
}

enum Four {
  _1,
  _2,
  _3,
  _4,
}
impl Four {
  const VALUES: [&Four; 4] = [&Four::_1, &Four::_2, &Four::_3, &Four::_4,];
}

enum Five {
  _1,
  _2,
  _3,
  _4,
  _5,
}
impl Five {
  const VALUES: [&Five; 5] = [&Five::_1, &Five::_2, &Five::_3, &Five::_4, &Five::_5,];
}
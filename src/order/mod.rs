use crate::{
  relation::{Closure, Equivalence},
  set::{Zero, One, Two, Three, Four, Five,
    partition::{Part, Partition}}
  };
use std::collections::HashSet;
use std::hash::Hash;

pub fn le<'a, A : Eq + Hash>(a: &Part<&A>, b: &Part<&A>) -> bool {
  let connection_set_a = Part::<&A>::get_connections(a.first()).equivalence();
  let connection_set_b = Part::<&A>::get_connections(b.first()).equivalence();
  connection_set_a.is_subset(&connection_set_b)
}

pub trait Join<A> {
  fn join(&self, a: &A) -> A;
}

impl<'a> Join<Part<&'a Zero>> for Part<&'a Zero> {
  fn join(&self, _: &Part<&'a Zero>) -> Part<&'a Zero> {
    self.clone()
  }
}

impl<'a> Join<Part<&'a One>> for Part<&'a One> {
  fn join(&self, a: &Part<&'a One>) -> Part<&'a One> {
    let connection_set_self: HashSet<(&One, &One)> = Self::get_connections(self.first()).equivalence();
    let connection_set_a: HashSet<(&One, &One)> = Self::get_connections(a.first()).equivalence();
    let connection_set_union: HashSet<(&One, &One)> = connection_set_self.union(&connection_set_a).cloned().collect();
    One::SET
      .partition()
      .iter()
      .find(|&p| { Self::get_connections(p.first()).equivalence() == connection_set_union })
      .map_or(self, |x| x)
      .clone()
  }
}

impl<'a> Join<Part<&'a Two>> for Part<&'a Two> {
  fn join(&self, a: &Part<&'a Two>) -> Part<&'a Two> {
    let connection_set_self: HashSet<(&Two, &Two)> = Self::get_connections(self.first()).equivalence();
    let connection_set_a: HashSet<(&Two, &Two)> = Self::get_connections(a.first()).equivalence();
    let connection_set_union: HashSet<(&Two, &Two)> = connection_set_self.union(&connection_set_a).cloned().collect();
    Two::SET
      .partition()
      .iter()
      .find(|&p| { Self::get_connections(p.first()).equivalence() == connection_set_union })
      .map_or(self, |x| x)
      .clone()
  }
}

impl<'a> Join<Part<&'a Three>> for Part<&'a Three> {
  fn join(&self, a: &Part<&'a Three>) -> Part<&'a Three> {
    let connection_set_self: HashSet<(&Three, &Three)> = Self::get_connections(self.first()).equivalence();
    let connection_set_a: HashSet<(&Three, &Three)> = Self::get_connections(a.first()).equivalence();
    let connection_set_union: HashSet<(&Three, &Three)> = connection_set_self.union(&connection_set_a).cloned().collect();
    Three::SET
      .partition()
      .iter()
      .find(|&p| { Self::get_connections(p.first()).equivalence() == connection_set_union })
      .map_or(self, |x| x)
      .clone()
  }
}

impl<'a> Join<Part<&'a Four>> for Part<&'a Four> {
  fn join(&self, a: &Part<&'a Four>) -> Part<&'a Four> {
    let connection_set_self: HashSet<(&Four, &Four)> = Self::get_connections(self.first()).equivalence();
    let connection_set_a: HashSet<(&Four, &Four)> = Self::get_connections(a.first()).equivalence();
    let connection_set_union: HashSet<(&Four, &Four)> = connection_set_self.union(&connection_set_a).cloned().collect();
    Four::SET
      .partition()
      .iter()
      .find(|&p| { Self::get_connections(p.first()).equivalence() == connection_set_union })
      .map_or(self, |x| x)
      .clone()
  }
}

impl<'a> Join<Part<&'a Five>> for Part<&'a Five> {
  fn join(&self, a: &Part<&'a Five>) -> Part<&'a Five> {
    let connection_set_self: HashSet<(&Five, &Five)> = Self::get_connections(self.first()).equivalence();
    let connection_set_a: HashSet<(&Five, &Five)> = Self::get_connections(a.first()).equivalence();
    let connection_set_union: HashSet<(&Five, &Five)> = connection_set_self.union(&connection_set_a).cloned().collect();
    Five::SET
      .partition()
      .iter()
      .find(|&p| { Self::get_connections(p.first()).equivalence() == connection_set_union })
      .map_or(self, |x| x)
      .clone()
  }
}
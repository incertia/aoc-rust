use aoc::{Solution, impl_solver};

use std::collections::{BTreeSet, HashSet};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Turn {
  Left,
  Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Instruction {
  turn: Turn,
  dist: i64,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let turn = match chars.next().ok_or(())? {
      'L' => Ok(Turn::Left),
      'R' => Ok(Turn::Right),
      _ => unreachable!(),
    }?;
    let dist = chars.as_str().parse().map_err(|_| ())?;
    Ok(Self { turn, dist })
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
  E,
  S,
  W,
  N,
}

impl Default for Direction {
  fn default() -> Self {
    Self::N
  }
}

impl Direction {
  pub fn cw(&self) -> Self {
    match self {
      Self::E => Self::S,
      Self::S => Self::W,
      Self::W => Self::N,
      Self::N => Self::E,
    }
  }

  pub fn ccw(&self) -> Self {
    match self {
      Self::E => Self::N,
      Self::S => Self::E,
      Self::W => Self::S,
      Self::N => Self::W,
    }
  }

  pub fn turn(&self, dir: Turn) -> Self {
    match dir {
      Turn::Left => self.ccw(),
      Turn::Right => self.cw(),
    }
  }

  pub fn to_vec(&self) -> Point {
    match self {
      Direction::E => Point { x: 1, y: 0 },
      Direction::S => Point { x: 0, y: -1 },
      Direction::W => Point { x: -1, y: 0 },
      Direction::N => Point { x: 0, y: 1 },
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  pub fn walk(&self, dir: Direction, dist: i64) -> impl Iterator<Item = Self> + Clone {
    let Self { x, y } = *self;
    let vec = dir.to_vec();
    (1..=dist).map(move |d| Self {
      x: x + d * vec.x,
      y: y + d * vec.y,
    })
  }

  pub fn taxicab(&self) -> i64 {
    self.x.abs() + self.y.abs()
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State<Collection> {
  dir: Direction,
  pos: Point,
  history: Collection,
}

impl Default for State<()> {
  fn default() -> Self {
    Self {
      dir: Default::default(),
      pos: Default::default(),
      history: (),
    }
  }
}

impl Default for State<Vec<Point>> {
  fn default() -> Self {
    let mut history = Vec::with_capacity(2048);
    history.push(Default::default());
    Self {
      dir: Default::default(),
      pos: Default::default(),
      history,
    }
  }
}

impl Default for State<BTreeSet<Point>> {
  fn default() -> Self {
    Self {
      dir: Default::default(),
      pos: Default::default(),
      history: [Default::default()].into(),
    }
  }
}

impl Default for State<HashSet<Point>> {
  fn default() -> Self {
    let mut history = HashSet::with_capacity(2048);
    history.insert(Default::default());
    Self {
      dir: Default::default(),
      pos: Default::default(),
      history,
    }
  }
}

trait Contains<T> {
  fn contains(&self, item: &T) -> bool;
}

impl<T> Contains<T> for () {
  #[inline(always)]
  fn contains(&self, _: &T) -> bool {
    false
  }
}

impl<T: PartialEq> Contains<T> for Vec<T> {
  #[inline(always)]
  fn contains(&self, item: &T) -> bool {
    self.as_slice().contains(item)
  }
}

impl<T: Ord + PartialEq> Contains<T> for BTreeSet<T> {
  #[inline(always)]
  fn contains(&self, item: &T) -> bool {
    self.contains(item)
  }
}

impl<T: std::hash::Hash + Eq> Contains<T> for HashSet<T> {
  #[inline(always)]
  fn contains(&self, item: &T) -> bool {
    HashSet::contains(self, item)
  }
}

impl<Collection> State<Collection> {
  pub fn step_a(self, inst: Instruction) -> Self {
    let State { dir, pos, history } = self;
    let dir = dir.turn(inst.turn);
    let pos = pos.walk(dir, inst.dist).last().unwrap();
    Self { dir, pos, history }
  }

  pub fn step_b(self, inst: Instruction) -> Result<Self, Point>
  where
    Collection: Extend<Point> + Contains<Point>,
  {
    let State {
      dir,
      pos,
      mut history,
    } = self;
    let dir = dir.turn(inst.turn);
    let new = pos.walk(dir, inst.dist);
    let pos = new.clone().last().unwrap();
    match new.clone().find(|p| history.contains(p)) {
      Some(new_pos) => Err(new_pos),
      None => {
        history.extend(new);
        Ok(Self { dir, pos, history })
      }
    }
  }
}

fn parse(input: &[u8]) -> Input {
  Input {
    insts: input
      .split(|c| *c == b',')
      .map(|s| {
        unsafe { str::from_utf8_unchecked(s) }
          .trim()
          .parse()
          .expect("correctly formatted input")
      })
      .collect(),
  }
}

struct Input {
  insts: Box<[Instruction]>,
}

fn solve_a(input: &Input) -> Solution {
  let state = input
    .insts
    .iter()
    .fold(State::<()>::default(), |st, inst| st.step_a(*inst));

  Solution::Number(state.pos.taxicab())
}

fn solve_b(input: &Input) -> Solution {
  let state = input
    .insts
    .iter()
    .try_fold(State::<BTreeSet<_>>::default(), |st, inst| st.step_b(*inst));

  unsafe { Solution::Number(state.err().unwrap_unchecked().taxicab()) }
}

impl_solver!(1, parse, solve_a, solve_b);

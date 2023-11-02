use std::{ops::{Add, AddAssign, Sub}, cmp::Ordering, mem::{replace, swap}};

use crate::*;

fn clamp_and_overflow<T: Ord + Sub + Copy + num_traits::Zero>(value: &mut T, min: T, max: T) -> T {
  assert!(min <= max);
  if value < min {
    // d = negative
    let d = min - value;
    *value = min;
    d
  } else if value > max {
    // d = posititive
    let d = value - max;
    *value = max;
    d
  } else {
    Default::default();
  }
}

struct Stat<F>(isize, isize, isize, F) where F: Fn(&mut Self, isize);
impl<F> Stat<F> where F: Fn(&mut Self, isize) {
    
}
impl<F> AddAssign<isize> for Stat<F> where F: Fn(&mut Self, isize) {
    fn add_assign(&mut self, rhs: isize) {
        self.0 = 
    }
}

pub trait Creature {
    /// Handles the creature's next tick.
    /// Does some \[...].
    /// The first return value is whether or not the screen needs to be redrawn
    /// and the second is how the creature died this tick (e.g. passive attacks or spots), if so.
    fn next_tick(&mut self) -> (bool, Option<Death>);

    /// Handles an attack from another creature. Returns how that attack killed this creature (if so)
    /// and a reverse attack.
    fn attack(&mut self, attacker: Option<&dyn Creature>, damage_type: DamageType) -> (Option<Death>, Option<(isize, DamageT)>)
}

struct CCD {
    x: usize,
    y: usize,
    hp: Stat<isize>, // negative if mortally wounded
    mp: isize,
}

struct Player(CCD);
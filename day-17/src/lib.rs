use itertools::Itertools;
use std::ops;

pub fn part_2(t: Target) -> usize {
    t.clone()
        .possible_iter()
        .filter(|&vel| t.clone().check_velocity(vel))
        .count()
}

#[derive(Clone)]
pub struct Target {
    pub x: ops::RangeInclusive<i32>,
    pub y: ops::RangeInclusive<i32>,
}

impl Target {
    fn possible_iter(self) -> impl Iterator<Item = (i32, i32)> {
        (0..*self.x.end() + 1).cartesian_product(*self.y.start()..-*self.y.start())
    }

    fn is_above(&self) -> bool {
        *self.y.start() > 0
    }

    fn at_zero(&self) -> bool {
        self.x.contains(&0) && self.y.contains(&0)
    }

    fn check_velocity(mut self, vel: (i32, i32)) -> bool {
        if self.at_zero() {
            return true;
        }
        if self.is_above() {
            return false;
        }

        self.x = self.x.start() - vel.0..=self.x.end() - vel.0;
        self.y = self.y.start() - vel.1..=self.y.end() - vel.1;
        let vel = (vel.0.signum() * (vel.0.abs() - 1), vel.1 - 1);

        self.check_velocity(vel)
    }
}

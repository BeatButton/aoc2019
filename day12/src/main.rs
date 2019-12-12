use fnv::{FnvHashSet as HashSet, FnvHasher};
use rayon::prelude::*;

use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

const INPUT: &str = include_str!("input");

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    #[allow(unused)]
    fn kinetic_energy(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    #[allow(unused)]
    fn potential_energy(&self) -> i64 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    #[allow(unused)]
    fn energy(&self) -> i64 {
        self.kinetic_energy() * self.potential_energy()
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct System {
    moons: Vec<Moon>,
}

impl System {
    #[allow(unused)]
    fn energy(&self) -> i64 {
        self.moons.par_iter().map(|moon| moon.energy()).sum()
    }

    fn step(&mut self) {
        let update_one_velocity = |new_vel: &mut i64, new_pos: &mut i64, old_pos: i64| {
            *new_vel += match old_pos.cmp(&new_pos) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
        };

        let old_moons = self.moons.clone();
        self.moons.par_iter_mut().for_each(|moon| {
            for old_moon in &old_moons {
                update_one_velocity(
                    &mut moon.velocity.x,
                    &mut moon.position.x,
                    old_moon.position.x,
                );
                update_one_velocity(
                    &mut moon.velocity.y,
                    &mut moon.position.y,
                    old_moon.position.y,
                );
                update_one_velocity(
                    &mut moon.velocity.z,
                    &mut moon.position.z,
                    old_moon.position.z,
                );
            }
            moon.apply_velocity();
        });
    }

    #[allow(unused)]
    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step()
        }
    }
}

fn main() {
    let moons: Vec<Moon> = INPUT
        .lines()
        .map(|line| {
            let mut line = line.chars();
            line.by_ref().take_while(|&ch| ch != '=').for_each(|_| {});

            let x: String = line.by_ref().take_while(|&ch| ch != ',').collect();
            let x: i64 = x.parse().unwrap();

            line.by_ref().take_while(|&ch| ch != '=').for_each(|_| {});

            let y: String = line.by_ref().take_while(|&ch| ch != ',').collect();
            let y: i64 = y.parse().unwrap();

            line.by_ref().take_while(|&ch| ch != '=').for_each(|_| {});

            let z: String = line.by_ref().take_while(|&ch| ch != '>').collect();
            let z: i64 = z.parse().unwrap();

            let position = Position { x, y, z };
            let velocity = Default::default();
            Moon { position, velocity }
        })
        .collect();
    let mut system = System { moons };
    let mut seen_states: HashSet<u64> = HashSet::default();
    for step in 0.. {
        let mut hasher = FnvHasher::default();
        system.hash(&mut hasher);
        if !seen_states.insert(hasher.finish()) {
            println!("{}", step);
            break;
        }
        system.step();
    }
}

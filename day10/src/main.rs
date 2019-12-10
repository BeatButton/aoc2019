use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use itertools::Itertools;

use std::{
    cmp::Ordering,
    f64::consts::PI,
    hash::{Hash, Hasher},
};

const INPUT: &str = include_str!("input");

const EPSILON: f64 = 0.001;

#[derive(Clone, Copy)]
struct Angle(f64);

impl PartialEq<Angle> for Angle {
    fn eq(&self, other: &Angle) -> bool {
        (self.0 - other.0).abs() <= EPSILON
    }
}

impl Eq for Angle {}

impl Hash for Angle {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (self.0 as i64).hash(state)
    }
}

impl PartialOrd<Angle> for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

fn are_coprime(x: i64, y: i64) -> bool {
    let mut x = x.abs() as usize;
    let mut y = y.abs() as usize;
    if y < x {
        std::mem::swap(&mut x, &mut y);
    }
    let lim = (y as f64).powf(0.5).ceil() as usize;

    if x % 2 == 0 && y % 2 == 0 {
        return false;
    }

    for div in (3..=lim).step_by(2) {
        if x % div == 0 && y % div == 0 {
            return false;
        }
    }
    true
}

fn get_rotations(x: i64, y: i64) -> [(i64, i64); 4] {
    [(x, y), (-x, y), (x, -y), (-x, -y)]
}

fn main() {
    let mut asteroids: HashSet<(i64, i64)> = INPUT
        .split_ascii_whitespace()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .flatten()
        .collect();
    let dim = INPUT.bytes().take_while(|&b| b != b'\n').count() as i64;

    let mut max = 0;
    let mut max_coord = (0, 0);

    for &(x, y) in &asteroids {
        let mut seen_from_here = asteroids.clone();
        seen_from_here.remove(&(x, y));
        for (p1, p2) in (0..dim)
            .cartesian_product(0..dim)
            .filter(|&(a, b)| are_coprime(a, b))
        {
            for (dx, dy) in get_rotations(p1, p2).into_iter() {
                let (mut curr_x, mut curr_y) = (x + dx, y + dy);
                let mut seen_one = false;
                while curr_x >= 0 && curr_x < dim && curr_y >= 0 && curr_y < dim {
                    if asteroids.contains(&(curr_x, curr_y)) {
                        if seen_one {
                            seen_from_here.remove(&(curr_x, curr_y));
                        } else {
                            seen_one = true;
                        }
                    }
                    curr_x += dx;
                    curr_y += dy;
                }
            }
        }
        if seen_from_here.len() > max {
            max = seen_from_here.len();
            max_coord = (x, y);
        }
    }

    println!("Saw {} from {},{}", max, max_coord.0, max_coord.1);

    asteroids.remove(&max_coord);

    let mut asteroids_by_theta: HashMap<Angle, Vec<(i64, i64)>> = HashMap::default();

    let (x1, y1) = (max_coord.0 as f64, max_coord.1 as f64);
    for asteroid in asteroids {
        let (x2, y2) = (asteroid.0 as f64, asteroid.1 as f64);
        let mut theta = (y2 - y1).atan2(x2 - x1) * 180. / PI + 90.;
        if theta < 0. {
            theta += 360.;
        }
        asteroids_by_theta
            .entry(Angle(theta))
            .or_insert(vec![])
            .push(asteroid);
    }

    for asteroids_on_line in asteroids_by_theta.values_mut() {
        asteroids_on_line.sort_unstable_by_key(|&(x2, y2)| {
            -((x1 + x2 as f64).powf(2.) + (y1 + y2 as f64).powf(2.)).powf(0.5) as i64
        })
    }

    let mut thetas: Vec<Angle> = asteroids_by_theta.keys().cloned().collect();
    thetas.sort_unstable_by(|a, b| a.partial_cmp(b).expect("a theta was NaN"));

    let mut asteroids_destroyed = 0;

    let mut done = false;
    while !done {
        for theta in &thetas {
            let asteroids_on_line = asteroids_by_theta.get_mut(theta).unwrap();
            if let Some(asteroid) = asteroids_on_line.pop() {
                asteroids_destroyed += 1;
                if asteroids_destroyed == 200 {
                    println!("Destroyed 200th asteroid at {},{}", asteroid.0, asteroid.1);
                    println!("Answer: {}", asteroid.0 * 100 + asteroid.1);
                    done = true;
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coprime_units() {
        assert!(are_coprime(1, 1));
        assert!(are_coprime(1, -1));
        assert!(are_coprime(-1, -1));
        assert!(are_coprime(1, 0));
        assert!(are_coprime(-1, 0));
        assert!(!are_coprime(0, 0));
    }
    #[test]
    fn test_coprime_composites() {
        assert!(!are_coprime(36, 8));
    }
    #[test]

    fn test_coprime_prime_composite() {
        assert!(!are_coprime(15, 3));
    }
}

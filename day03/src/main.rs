use fnv::FnvHashSet as HashSet;

use std::hash::{Hash, Hasher};

static INPUT: &str = include_str!("input");

#[derive(Clone, Copy, Debug, Eq)]
struct Point {
    x: i32,
    y: i32,
    d: usize,
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn segments_to_point_set<'a, I>(segments: I) -> HashSet<Point>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut points = HashSet::default();
    let (mut x, mut y) = (0, 0);
    let mut d = 0;
    for segment in segments {
        let mut iter = segment.chars();
        let direction = iter.next().expect("wire had no direction");
        let distance = iter
            .collect::<String>()
            .parse::<usize>()
            .expect("wire had bad distance");
        let (dx, dy) = match direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            other => panic!("Unrecognized direction {}", other),
        };
        for _ in 0..distance {
            x += dx;
            y += dy;
            d += 1;
            let p = Point { x, y, d };

            points.insert(p);
        }
    }
    points
}

fn main() {
    let mut iter = INPUT.split('\n');
    let wire1 = segments_to_point_set(iter.next().expect("no 1st wire").split(','));
    let wire2 = segments_to_point_set(iter.next().expect("no 2nd wire").split(','));
    let intersections = wire1.intersection(&wire2);
    let answer = intersections
        .map(|p| {
            let p1 = wire1.get(p).expect("wire 1 missing intersection");
            let p2 = wire2.get(p).expect("wire 2 missing intersection");
            p1.d + p2.d
        })
        .min()
        .expect("no intersections");
    println!("{}", answer);
}

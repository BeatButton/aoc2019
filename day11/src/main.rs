use fnv::FnvHashMap as HashMap;

use intcode::{Computer, Rx, Tx};

const PROGRAM: [i64; 652] = include!("input");

struct Robot {
    rx: Rx,
    tx: Tx,
    facing: Direction,
    x: i64,
    y: i64,
    painted_panels: HashMap<(i64, i64), i64>,
}

impl Robot {
    fn new(program: Vec<i64>) -> Self {
        let (cpu, rx, tx) = Computer::from_data_with_io(program);
        cpu.run_threaded();
        let mut painted_panels = HashMap::default();
        painted_panels.insert((0, 0), 1);
        Self {
            rx,
            tx,
            facing: Direction::North,
            x: 0,
            y: 0,
            painted_panels: painted_panels,
        }
    }

    fn run(&mut self) {
        while let Ok(()) = self
            .tx
            .send(*self.painted_panels.get(&(self.x, self.y)).unwrap_or(&0))
        {
            match self.rx.recv() {
                Ok(0) => {
                    let panel = self.painted_panels.entry((self.x, self.y)).or_insert(0);
                    *panel = 0;
                }
                Ok(1) => {
                    let panel = self.painted_panels.entry((self.x, self.y)).or_insert(0);
                    *panel = 1;
                }
                Ok(err) => panic!("Received unexpected directive {}", err),
                Err(_) => break,
            };

            match self.rx.recv().unwrap() {
                0 => self.facing = self.facing.left(),
                1 => self.facing = self.facing.right(),
                err => panic!("Received unexpected directive {}", err),
            };

            let (dx, dy) = match self.facing {
                Direction::North => (0, 1),
                Direction::East => (1, 0),
                Direction::South => (0, -1),
                Direction::West => (-1, 0),
            };

            self.x += dx;
            self.y += dy;
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

fn main() {
    let mut robot = Robot::new(PROGRAM.to_vec());
    robot.run();
    let mut white_panels: Vec<(i64, i64)> = robot
        .painted_panels
        .iter()
        .filter(|&(_, &paint)| paint == 1)
        .map(|(&coord, _)| coord)
        .collect();

    let min_x = white_panels.iter().cloned().map(|(x, _)| x).min().unwrap();
    let min_y = white_panels.iter().cloned().map(|(_, y)| y).min().unwrap();

    for (x, y) in &mut white_panels {
        *x -= min_x;
        *y -= min_y;
    }

    let max_x = white_panels.iter().cloned().map(|(x, _)| x).max().unwrap();
    let max_y = white_panels.iter().cloned().map(|(_, y)| y).max().unwrap();

    let mut canvas = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

    for (x, y) in white_panels {
        canvas[y as usize][x as usize] = 1;
    }

    let output = canvas
        .into_iter()
        .rev()
        .map(|line| {
            line.into_iter()
                .map(|n| if n == 1 { 'X' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", output);
}

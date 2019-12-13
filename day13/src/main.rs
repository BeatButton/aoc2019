use fnv::FnvHashMap as HashMap;
use intcode::{Computer, Rx, Tx};

use std::{cmp::Ordering, sync::mpsc::TryRecvError};

const PROGRAM: [i64; 2320] = include!("input");

struct Cabinet {
    rx: Rx,
    tx: Tx,
    entities: HashMap<(usize, usize), Entity>,
    score: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Entity {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Invalid,
}

impl From<i64> for Entity {
    fn from(num: i64) -> Self {
        use Entity::*;
        match num {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => Invalid,
        }
    }
}

impl From<Entity> for char {
    fn from(entity: Entity) -> Self {
        use Entity::*;
        match entity {
            Empty => ' ',
            Wall => '|',
            Block => 'X',
            Paddle => '-',
            Ball => 'O',
            _ => '?',
        }
    }
}

impl Cabinet {
    fn from_program(program: Vec<i64>) -> Self {
        let (cpu, rx, tx) = Computer::from_data_with_io(program);
        cpu.run_threaded();
        Self {
            tx,
            rx,
            entities: HashMap::default(),
            score: 0,
        }
    }

    fn step(&mut self) -> bool {
        match self.rx.try_recv() {
            Ok(x) => {
                let y = self.rx.recv().expect("failed to get y") as usize;
                if x == -1 {
                    assert_eq!(y, 0);
                    self.score = self.rx.recv().expect("failed to get score") as usize;
                } else {
                    let id = self.rx.recv().expect("failed to get id");
                    let tile = self
                        .entities
                        .entry((x as usize, y))
                        .or_insert(Entity::Empty);
                    *tile = id.into();
                }
                true
            }
            Err(TryRecvError::Empty) => {
                if let Some(((ball_x, _), _)) = self
                    .entities
                    .iter()
                    .find(|&(_, &entity)| entity == Entity::Ball)
                {
                    if let Some(((paddle_x, _), _)) = self
                        .entities
                        .iter()
                        .find(|&(_, &entity)| entity == Entity::Paddle)
                    {
                        let signal = match ball_x.cmp(&paddle_x) {
                            Ordering::Greater => 1,
                            Ordering::Less => -1,
                            Ordering::Equal => 0,
                        };
                        let _ = self.tx.try_send(signal);
                    }
                }
                true
            }
            Err(TryRecvError::Disconnected) => false,
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

fn main() {
    let mut program = PROGRAM.to_vec();
    program[0] = 2;
    let mut cabinet = Cabinet::from_program(program);
    cabinet.run();
    println!("{}", cabinet.score);
}

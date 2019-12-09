use fnv::FnvHashMap as HashMap;

use std::{sync::mpsc, thread};

const POWERS_OF_10_FROM_100: [i64; 3] = [100, 1000, 10000];

pub type Rx = mpsc::Receiver<i64>;
pub type Tx = mpsc::Sender<i64>;

#[derive(Debug)]
pub struct Computer {
    pub data: HashMap<usize, i64>,
    pub idx: usize,
    pub instruction: i64,
    pub relative_base: i64,
    input: Rx,
    output: Tx,
}

impl Computer {
    pub fn from_data(data: Vec<i64>) -> Self {
        let (output, input) = mpsc::channel();
        Self::from_data_with_custom_io(data, input, output)
    }

    pub fn from_data_with_io(data: Vec<i64>) -> (Self, Rx, Tx) {
        let (output, extern_rx) = mpsc::channel();
        let (extern_tx, input) = mpsc::channel();

        (
            Self::from_data_with_custom_io(data, input, output),
            extern_rx,
            extern_tx,
        )
    }

    pub fn from_data_with_custom_io(data: Vec<i64>, input: Rx, output: Tx) -> Self {
        let mut hash_data = HashMap::default();
        for (idx, value) in data.into_iter().enumerate() {
            hash_data.insert(idx, value);
        }
        Self {
            data: hash_data,
            idx: 0,
            instruction: 0,
            relative_base: 0,
            input,
            output,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.instruction = self.get(self.idx);
            let opcode = self.instruction % 100;
            match opcode {
                // ADD
                1 => {
                    let lhs = self.get_param(0);
                    let rhs = self.get_param(1);

                    let dest = self.get_index(2);

                    self.set(dest, lhs + rhs);
                    self.idx += 4;
                }
                // MULTIPLY
                2 => {
                    let lhs = self.get_param(0);
                    let rhs = self.get_param(1);

                    let dest = self.get_index(2);

                    self.set(dest, lhs * rhs);
                    self.idx += 4;
                }
                // INPUT
                3 => {
                    let in_value: i64 = self.input.recv().expect("failed to get input");
                    let dest = self.get_index(0);

                    self.set(dest, in_value);
                    self.idx += 2;
                }
                // OUTPUT
                4 => {
                    let out = self.get_param(0);
                    self.output.send(out).expect("failed to send output");

                    self.idx += 2;
                }
                // JUMP IF TRUE
                5 => {
                    let param = self.get_param(0);

                    if param != 0 {
                        self.idx = self.get_param(1) as usize;
                    } else {
                        self.idx += 3;
                    }
                }
                // JUMP IF FALSE
                6 => {
                    let param = self.get_param(0);

                    if param == 0 {
                        self.idx = self.get_param(1) as usize;
                    } else {
                        self.idx += 3;
                    }
                }
                // LESS THAN
                7 => {
                    let lhs = self.get_param(0);
                    let rhs = self.get_param(1);

                    let dest = self.get_index(2);

                    self.set(dest, (lhs < rhs) as i64);

                    self.idx += 4;
                }
                // EQUALS
                8 => {
                    let lhs = self.get_param(0);
                    let rhs = self.get_param(1);

                    let dest = self.get_index(2);

                    self.set(dest, (lhs == rhs) as i64);

                    self.idx += 4;
                }
                // RELATIVE BASE OFFSET
                9 => {
                    let param = self.get_param(0);

                    self.relative_base += param;

                    self.idx += 2;
                }
                99 => break,
                err => panic!("Unrecognized opcode {}", err),
            }
        }
    }

    fn get_param(&self, param_idx: usize) -> i64 {
        let idx = self.get_index(param_idx);
        self.get(idx)
    }

    fn get_index(&self, param_idx: usize) -> usize {
        let param_mode = self.instruction / POWERS_OF_10_FROM_100[param_idx] % 10;
        match param_mode {
            0 => self.get(self.idx + param_idx + 1) as usize,
            1 => self.idx + param_idx + 1,
            2 => (self.get(self.idx + param_idx + 1) as i64 + self.relative_base) as usize,
            err => panic!("Unrecognized parameter mode {}", err),
        }
    }

    fn get(&self, idx: usize) -> i64 {
        *self.data.get(&idx).unwrap_or(&0)
    }

    fn set(&mut self, idx: usize, value: i64) {
        let stored = self.data.entry(idx).or_insert(0);
        *stored = value;
    }

    pub fn run_threaded(mut self) -> thread::JoinHandle<Self> {
        thread::spawn(move || {
            self.run();
            self
        })
    }
}

use std::sync::mpsc;

const POWERS_OF_10_FROM_100: [i64; 2] = [100, 1000];

pub type Rx = mpsc::Receiver<i64>;
pub type Tx = mpsc::Sender<i64>;

#[derive(Debug)]
pub struct Computer {
    pub data: Vec<i64>,
    pub idx: usize,
    pub instruction: i64,
    input: Rx,
    output: Tx,
}

impl Computer {
    pub fn from_data(data: Vec<i64>) -> Self {
        let (output, input) = mpsc::channel();
        Self {
            data,
            idx: 0,
            instruction: 0,
            input,
            output,
        }
    }

    pub fn from_data_with_io(data: Vec<i64>) -> (Self, Rx, Tx) {
        let (output, extern_rx) = mpsc::channel();
        let (extern_tx, input) = mpsc::channel();

        (
            Self {
                data,
                idx: 0,
                instruction: 0,
                input,
                output,
            },
            extern_rx,
            extern_tx,
        )
    }

    pub fn from_data_with_custom_io(data: Vec<i64>, input: Rx, output: Tx) -> Self {
        Self {
            data,
            idx: 0,
            instruction: 0,
            input,
            output,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.instruction = self.data[self.idx];
            let opcode = self.instruction % 100;
            match opcode {
                // ADD
                1 => {
                    let lhs = self.get_param(0);
                    let rhs = self.get_param(1);

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = lhs + rhs;
                    self.idx += 4;
                }
                // MULTIPLY
                2 => {
                    let lhs = self.get_param(0);
                    let rhs = self.get_param(1);

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = lhs * rhs;
                    self.idx += 4;
                }
                // INPUT
                3 => {
                    let in_value: i64 = self.input.recv().expect("failed to get input");
                    let dest = self.data[self.idx + 1] as usize;

                    self.data[dest] = in_value;
                    self.idx += 2;
                }
                // OUTPUT
                4 => {
                    self.output
                        .send(self.get_param(0))
                        .expect("failed to send output");

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

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = (lhs < rhs) as i64;

                    self.idx += 4;
                }
                // EQUALS
                8 => {
                    let lhs = self.get_param(0);
                    let rhs = self.get_param(1);

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = (lhs == rhs) as i64;

                    self.idx += 4;
                }
                99 => break,
                err => panic!("Unrecognized opcode {}", err),
            }
        }
    }

    fn get_param(&self, param_idx: usize) -> i64 {
        match self.instruction / unsafe { POWERS_OF_10_FROM_100.get_unchecked(param_idx) } % 10 {
            0 => self.data[self.data[self.idx + param_idx + 1] as usize],
            1 => self.data[self.idx + param_idx + 1],
            err => panic!("Unrecognized parameter mode {}", err),
        }
    }
}

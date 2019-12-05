macro_rules! input {
    () => {
        input!("")
    };
    ( $($e:expr),* ) => {{
        eprint!($($e),*);
        let mut stderr = ::std::io::stderr();
        ::std::io::Write::flush(&mut stderr).expect("Failed to flush stderr");
        let stdin = ::std::io::stdin();
        let mut s = String::new();
        stdin.read_line(&mut s).expect("Failed to read stdin");
        s
    }};
}

pub struct Computer {
    pub data: Vec<i64>,
    pub idx: usize,
}

impl Computer {
    pub fn from_data(data: Vec<i64>) -> Self {
        Self { data, idx: 0 }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.data[self.idx];
            let opcode = instruction % 100;
            match opcode {
                // ADD
                1 => {
                    let lhs = self.get_param(instruction, 0);
                    let rhs = self.get_param(instruction, 1);

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = lhs + rhs;
                    self.idx += 4;
                }
                // MULTIPLY
                2 => {
                    let lhs = self.get_param(instruction, 0);
                    let rhs = self.get_param(instruction, 1);

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = lhs * rhs;
                    self.idx += 4;
                }
                // INPUT
                3 => {
                    let in_value: i64 = input!("IN: ")
                        .trim()
                        .parse()
                        .expect("invalid integer entered");
                    let dest = self.data[self.idx + 1] as usize;

                    self.data[dest] = in_value;
                    self.idx += 2;
                }
                // OUTPUT
                4 => {
                    let out = self.get_param(instruction, 0);

                    println!("OUT: {}", out);

                    self.idx += 2;
                }
                // JUMP IF TRUE
                5 => {
                    let param = self.get_param(instruction, 0);
                    if param != 0 {
                        self.idx = self.get_param(instruction, 1) as usize;
                    } else {
                        self.idx += 3;
                    }
                }
                // JUMP IF FALSE
                6 => {
                    let param = self.get_param(instruction, 0);
                    if param == 0 {
                        self.idx = self.get_param(instruction, 1) as usize;
                    } else {
                        self.idx += 3;
                    }
                }
                // LESS THAN
                7 => {
                    let lhs = self.get_param(instruction, 0);
                    let rhs = self.get_param(instruction, 1);

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = (lhs < rhs) as i64;

                    self.idx += 4;
                }
                // EQUALS
                8 => {
                    let lhs = self.get_param(instruction, 0);
                    let rhs = self.get_param(instruction, 1);

                    let dest = self.data[self.idx + 3] as usize;

                    self.data[dest] = (lhs == rhs) as i64;

                    self.idx += 4;
                }
                99 => break,
                err => panic!("Unrecognized opcode {}", err),
            }
        }
    }

    fn get_param(&self, instruction: i64, param_idx: usize) -> i64 {
        match instruction / 10_i64.pow(2 + param_idx as u32) % 10 {
            0 => self.data[self.data[self.idx + param_idx + 1] as usize],
            1 => self.data[self.idx + param_idx + 1],
            err => panic!("Unrecognized parameter mode {}", err),
        }
    }
}

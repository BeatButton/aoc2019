pub fn run(data: &mut Vec<usize>) {
    let mut idx = 0;
    loop {
        match data[idx] {
            1 => {
                let lhs = data[data[idx + 1]];
                let rhs = data[data[idx + 2]];
                let dest = data[idx + 3];
                data[dest] = lhs + rhs;
                idx += 4;
            }
            2 => {
                let lhs = data[data[idx + 1]];
                let rhs = data[data[idx + 2]];
                let dest = data[idx + 3];
                data[dest] = lhs * rhs;
                idx += 4;
            }
            99 => break,
            err => panic!("Unrecognized opcode {}", err),
        }
    }
}

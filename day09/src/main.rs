use intcode::Computer;

const SOURCE: [i64; 973] = include!("input");

fn main() {
    let (cpu, rx, tx) = Computer::from_data_with_io(SOURCE.to_vec());
    cpu.run_threaded();
    tx.send(2).unwrap();
    while let Ok(output) = rx.recv() {
        println!("{}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let source = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (cpu, rx, _tx) = Computer::from_data_with_io(source.clone());
        cpu.run_threaded();
        let mut output = vec![];
        while let Ok(out) = rx.recv() {
            output.push(out);
        }
        assert_eq!(source, output);
    }

    #[test]
    fn test_2() {
        let (cpu, rx, _tx) =
            Computer::from_data_with_io(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        cpu.run_threaded();
        assert_eq!(rx.recv().unwrap().to_string().len(), 16)
    }
    #[test]
    fn test_3() {
        let source = vec![104, 1125899906842624, 99];
        let (cpu, rx, _tx) = Computer::from_data_with_io(source.clone());
        cpu.run_threaded();
        assert_eq!(rx.recv().unwrap(), source[1]);
    }
}

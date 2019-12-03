use intcode;

const SOURCE: [usize; 181] = include!("input");

fn main() {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut data = SOURCE.to_vec();
            data[1] = noun;
            data[2] = verb;
            intcode::run(&mut data);
            if data[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}

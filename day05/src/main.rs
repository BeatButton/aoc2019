use intcode::Computer;

const SOURCE: [i64; 678] = include!("input");

fn main() {
    let data = SOURCE.to_vec();
    let mut cpu = Computer::from_data(data);
    cpu.run();
}

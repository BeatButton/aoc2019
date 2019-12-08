use std::thread;

use intcode::Computer;

const SOURCE: [i64; 678] = include!("input");

fn main() {
    let data = SOURCE.to_vec();
    let (mut cpu, rx, tx) = Computer::from_data_with_io(data);
    thread::spawn(move || cpu.run());
    tx.send(5).expect("failed to send");
    let output = rx.recv().expect("failed to receive");
    println!("{}", output);
}

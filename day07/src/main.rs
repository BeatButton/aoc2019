use std::thread;

use itertools::Itertools;

use intcode::{Computer, Rx, Tx};

const SOURCE: [i64; 503] = include!("input");

fn main() {
    let mut max = 0;
    for settings_vec in (5..=9).permutations(5) {
        let mut last_signal = 0;
        let mut cpus: Vec<(Rx, Tx)> = Vec::new();
        for _ in 0..=4 {
            let (mut cpu, rx, tx) = Computer::from_data_with_io(SOURCE.to_vec());
            thread::spawn(move || cpu.run());
            cpus.push((rx, tx));
        }
        for i in 0..=4 {
            cpus[i]
                .1
                .send(settings_vec[i])
                .expect("failed to send initial setting");
        }

        let mut done = false;
        while !done {
            for i in 0..=4 {
                if let Err(_) = cpus[i].1.send(last_signal) {
                    done = true;
                    break;
                };
                last_signal = cpus[i].0.recv().expect("failed to receive signal");
            }
        }
        if last_signal > max {
            max = last_signal;
        }
    }
    println!("{}", max);
}

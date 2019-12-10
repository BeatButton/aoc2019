use std::sync::mpsc;

use itertools::Itertools;

use intcode::{Computer, Rx, Tx};

const SOURCE: [i64; 503] = include!("input");

fn main() {
    let mut max = 0;
    for settings_vec in (5..=9).permutations(5) {
        let mut last_signal = 0;

        let mut send: Vec<Option<Tx>> = vec![];
        let mut recv: Vec<Option<Rx>> = vec![];
        for _ in 0..6 {
            let (tx, rx) = mpsc::channel();
            send.push(Some(tx));
            recv.push(Some(rx));
        }

        for i in (0..5).rev() {
            let cpu = Computer::from_data_with_custom_io(
                SOURCE.to_vec(),
                recv[i].take().unwrap(),
                send[i + 1].take().unwrap(),
            );
            cpu.run_threaded();
            send[i].as_ref().unwrap().send(settings_vec[i]).unwrap();
        }

        let tx = send[0].take().unwrap();
        let rx = recv[5].take().unwrap();

        while let Ok(()) = tx.send(last_signal) {
            last_signal = rx.recv().expect("failed to receive signal");
        }
        if last_signal > max {
            max = last_signal;
        }
    }
    println!("{}", max);
}

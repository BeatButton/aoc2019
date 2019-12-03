#![warn(clippy::pedantic)]

const INPUT: &str = include_str!("input");

fn get_fuel(mass: u32) -> u32 {
    match (mass / 3).saturating_sub(2) {
        0 => 0,
        fuel => fuel + get_fuel(fuel),
    }
}

fn main() {
    println!(
        "{}",
        INPUT
            .lines()
            .map(|line| get_fuel(line.parse::<u32>().unwrap()))
            .sum::<u32>()
    );
}

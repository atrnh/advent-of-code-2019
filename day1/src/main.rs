use std::io::{self, BufRead};
use math::round;

fn main() {
    let mut sum: f64 = 0.0;

    for mass in io::stdin().lock().lines() {
        let mass = mass.expect("");
        let mass: f64 = match mass.trim().parse() {
            Ok(n) => calc_fuel(n),
            Err(_) => 0.0,
        };
        sum += mass;
    }

    println!("{}", sum);
}

fn calc_fuel(m: f64) -> f64 {
    let rnd = round::floor(m / 3.0, 0);
    rnd - 2.0
}

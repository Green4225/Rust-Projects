use std::f64::consts::PI;
use std::{io};

fn main() {
    let pi = PI;
     let mut line = String::new();
    let digit: usize;

    println!("Please enter how many digits you want PI to?");

    io::stdin()
    .read_line(&mut line)
    .expect("Failed to read input");

    digit = line
    .trim()
    .parse()
    .expect("Input was not an intger");

    println!("{:.1$}", pi, digit)




}

use std::{io};
fn main() {
    let my_number = fastrand::u16(1..1000);
    let mut correct = false;
    let mut user_number: u16;

    while correct == false {
        println!("Guess my number (1-1000)");

        let mut user_guess = String::new();

        io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read input");

        user_number = match user_guess
                .trim()
                .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_number == my_number {
            correct = true;

        }
        else if user_number < my_number{
            println!("Too low");

        }
        else if user_number > my_number{
            println!("Too high");

        }

    }
    println!("You Win!");



}

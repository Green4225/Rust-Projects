use std::{io};
fn main() {
    let my_number = fastrand::u16(..);
    let mut correct = false;
    let mut user_number: u16;

    while correct == false {
        println!("Guess my number (0-65535)");

        let mut user_guess = String::new();

        io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read input");

        user_number = user_guess
        .trim()
        .parse()
        .expect("Your input was not an intger");

        if user_number == my_number {
            correct = true;

        }
        else if user_number < my_number{
            println!("Your guess is lower than my number");

        }
        else if user_number > my_number{
            println!("Your guess is higher than my number");

        }
        
    }
    println!("You Win!");



}

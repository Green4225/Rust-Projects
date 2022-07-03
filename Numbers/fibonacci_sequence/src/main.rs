use std::{io};
fn main() {
    let mut f = 0;
    let mut f1 = 1;
    let mut f2 = 0;
    let mut temp;

    let mut correct_input = false;
    let mut user_n = 0;
    let mut n = 0;

    while correct_input == false {
        println!("Which number would like the Fibonacci Sequence to?");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

        let user_nl:i32 = match user_input
                .trim()
                .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        user_n = user_nl;
        n = user_n;
        correct_input = true;
    }
    while n != 0  {

        if user_n == 0 {
            break;
        }
        else if user_n == 1 {
            print!("{}", f2);
            break;
        }
        else if user_n == 2 {
            print!("{} {}", f2, f1);
            break;
        }
        else{

            if f == 0 {
                print!("{} {}", f2, f1);
                n = n - 2;
            }

            f = f2 + f1;
            print!(" {}", f);
            temp = f1;
            f1 = f;
            f2 = temp;

            n = n - 1;

        }
    }

    println!("");
    print!("Done! The Fibonacci Sequence to {} places", user_n);

}

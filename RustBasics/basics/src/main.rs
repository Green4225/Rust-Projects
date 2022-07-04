fn main() {
let s = String::from("Hello");
takes_ownership(&s);
println!("{}", s);

let x = 5;
let _y = x;
println!("{}", x);

let s1 = String::from("Hello");
let s2 = &s1;
println!("{} world!", s1);
println!("{} world!", s2);


let c1 = String::from("hello");
let len = calculate_length(&c1);
println!("The length of '{}' is {}.", c1, len);


let x = 5;
makes_copy(x);
println!("{}", x);
}

fn  takes_ownership(some_string: &String) {
    println!("{}", some_string);

}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length

}

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    println!("Please input your guess.");

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");

    let guess: u32 = line.trim().parse().expect("Wanted a positive number.");

    println!("Your guess was {}", guess);
    println!("The secret number was {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You got it!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
    }
}

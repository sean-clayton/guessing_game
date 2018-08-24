extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");
    print!("What is your guess? (1—10) ");
    io::stdout().flush().unwrap();

    let secret_number = rand::thread_rng().gen_range(1, 11);

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

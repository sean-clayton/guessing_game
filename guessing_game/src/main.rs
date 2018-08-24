extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    loop {
        print!("What is your guess? (1—10) ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");

        let guess: u32 = match line.trim().parse() {
            // If we have a valid number, just use the number
            Ok(num) if num >= 1 && num <= 10 => num,
            // We have zero.
            Ok(0) => {
                println!("0 is not equal to or greater than 1. Try again");
                continue;
            }
            // We have positive integer, but it's greater than 10
            Ok(num) => {
                println!("{} is greater than 10. Try again.", num);
                continue;
            }
            Err(_) => {
                println!("\"{}\" is not a positive integer. Try again.", line.trim());
                continue;
            }
        };

        println!("Your guess was {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("The secret number was {}! You got it!", secret_number);
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}

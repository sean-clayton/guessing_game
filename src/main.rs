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
            Ok(num) => num,
            Err(_) => {
                println!("\"{}\" is invalid. Try again.", line.trim());
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

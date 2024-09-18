extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let target = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Type your guess (1-100 range)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error while getting the number");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) if num > 0 && num < 101 => num,
            Ok(_) => {
                println!("The guess MUST be: 0 < guess < 101");
                continue;
            }
            Err(_) => {
                println!("You MUST type a number");
                continue;
            }
        };

        match guess.cmp(&target) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("High guess!"),
            Ordering::Less => println!("Low guess!"),
        }
    }
}

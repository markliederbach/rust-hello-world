use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive => exclusive

    loop {
        print!("Type your guess: ");
        io::stdout().flush().unwrap(); // needed to flush the buffer

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // .parse() infers the type based on the annotation
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "quit" {
                    println!("You quitter!");
                    break;
                };
                eprintln!("Either enter a number (1->100) or type 'quit' to exit");
                continue;
            }
        };
        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high, chief!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

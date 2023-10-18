use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                let diff = secret_number - guess;
                if diff <= 5 {
                    println!("A bit small!");
                } else {
                    println!("Too small!");
                }
            }
            Ordering::Greater => {
                let diff = guess - secret_number;
                if diff <= 5 {
                    println!("A bit big!");
                } else {
                    println!("Too big!");
                }
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

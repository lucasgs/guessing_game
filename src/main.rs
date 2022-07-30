use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! (1..100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    let max_tries = 3;
    let mut attemp = 0;
    while attemp < max_tries {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        attemp += 1;
    }
    if attemp >= max_tries {
        println!("The secret number was: {secret_number}");
    }
}

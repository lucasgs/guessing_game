use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! (1..100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    const MAX_TRIES: i8 = 3;
    let mut attemp = 0;
    while attemp < MAX_TRIES {
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

        match guess_number(guess, secret_number) {
            (false, msg) => {
                println!("{}", msg);
            }
            (true, msg) => {
                println!("{}", msg);
                break;
            }
        }

        attemp += 1;
        if attemp == MAX_TRIES {
            println!("The secret number was: {secret_number}");
        }
    }
}

fn guess_number(guess: u32, secret_number: u32) -> (bool, &'static str) {
    match guess.cmp(&secret_number) {
        Ordering::Less => return (false, "Too small!"),
        Ordering::Greater => return (false, "Too big!"),
        Ordering::Equal => return (true, "You win!"),
    };
}

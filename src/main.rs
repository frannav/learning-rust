use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("_ _ _ _ _ _ _ _ _ _ ");
    println!("- Guess the number!");
    println!("- Input your guess");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=4);

    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number, please");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win! 🏆");
                break;
            }
            Ordering::Greater => println!("Too high!"),
        }

        println!("- You guessed: {guess}");
    }
}

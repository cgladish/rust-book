use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let guess_hint = "Num should be between 1 and 100 (inclusive)";
    println!("Please input your guess. {guess_hint}");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) if (1..=100).contains(&num) => num,
            Ok(_) => {
                println!("Guess is out of range");
                println!("{guess_hint}");
                continue;
            }
            Err(err) => {
                println!("Error when parsing: {err}");
                println!("{guess_hint}");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!")
        }
    }
}

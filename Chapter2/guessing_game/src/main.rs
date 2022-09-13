use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("I am thinking of a number between 1 and 10");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small");
                continue;
        },
            Ordering::Greater => {
                println!("too big");
                continue;
        },
            Ordering::Equal => {
                println!("you win!");
                break;
                }        
            }
        }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess some number, then input your guess:");
    
    let comp_guess = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");


        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Current entry is not the correct format, please enter a number");
                continue;
            }
        };

        match guess.cmp(&comp_guess) {
            Ordering::Equal =>  {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        };
    }
}

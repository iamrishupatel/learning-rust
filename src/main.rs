use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // genearte a random number in the range 1 and 100 -
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess the number: ");

        // in rust variables are immutable by default
        // to make them mutable use `mut` keyword
        let mut guessed_vlaue = String::new();

        io::stdin()
            .read_line(&mut guessed_vlaue)
            .expect("failed to read input");

        // variable shadowing
        let guessed_vlaue: u32 = match guessed_vlaue.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                continue;
            }
        };

        match guessed_vlaue.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too high"),
        };
    }
}

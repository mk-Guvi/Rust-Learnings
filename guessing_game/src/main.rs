use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess The Number!");
        println!("Enter a number!");
        
        let mut guess: String = String::new(); // `String` is a type from the standard library that represents a growable string. `String::new()` creates a new, empty string. The `::` syntax indicates that `new` is an associated function of `String`.
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input"); // Reads user input from standard input and appends it to the `guess` string. `io::stdin()` returns a `Result`, an enum in Rust that represents success (`Ok`) or failure (`Err`). Here, we use `expect()` to crash the program with an error message if reading fails, rather than handling both cases.

        println!("You guessed: {}", guess);

        println!("Thanks"); // This will print on a new line since the input in `guess` includes a trailing newline (`\n`).

        let guess: u32 = match guess.trim().parse() {//shadowing guess variable to make rest of the code to be readonly.
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }; // This is an example of variable shadowing. The original `guess` variable is replaced with a parsed `u32`. It’s no longer mutable after this point.

        match guess.cmp(&secret_number) {
            // The `match` statement is similar to a switch-case but requires all conditions to be handled. The `cmp` method compares two values and returns an `Ordering` enum Custom Type, which can be `Greater`, `Less`, or `Equal`.
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You won!!!");
                break;
            }
        }
    }
}

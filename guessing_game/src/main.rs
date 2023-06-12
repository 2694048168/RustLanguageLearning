// prelude in Rust, e.g. 'println!'

// bring that type into scope explicitly with a 'use' statement.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// the 'main' function is the entry point into the program.
fn main() {
    // 'println!' is a macro that prints a string to the screen.
    println!("Guess the number from the random range(1-100)!");

    // 'cargo doc --open', and select 'rand' sidebar.
    let guess_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {guess_number}");

    loop {
        println!("Please enter your guess (1-100)");

        // 'let' create mutable variable, default is immutable without 'mut'
        // 'String::new' is a function that returns a new instance of a String.
        // '::' indicate associated function is a function that’s implemented on a type.
        let mut guess = String::new();

        // std::io::stdin()
        // Handling Potential Failure with Result.
        // 'rustfmt' tool to format the Rust source code.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.\n");

        // Rust allows us to shadow the previous value of guess with a new one.
        // Shadowing lets us reuse the guess variable name rather than
        // forcing us to create two unique variables.
        // The trim method eliminates '\n' or '\r\n';
        // The colon (:) after guess tells Rust we’ll annotate the variable’s type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // let guess: u32 = guess
        //     .trim()
        //     .parse()
        //     .expect("please entry a integer number.");

        // '{}' set of curly brackets is a placeholder, e.g. f-string in Python.
        println!("You guessed: {guess}");

        match guess.cmp(&guess_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    let secret_number = 42;
    println!("\nThe secret number for all is {secret_number}");
}

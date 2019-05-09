// import a specific package, in this case standard out.
use std::io::{self, Write};
//use std::io::Write;

fn main() {
    // Command line output
    println!("Guess the number!!");
    print!("Enter your number here: ");

    // Flushes and unwraps the new line.
    // Why is this necessary?
    // Terminals typically print new lines.
    io::stdout().flush().unwrap();

    // declare variable with let
    // mut = mutable
    // let foo = bar -> immutable.
    let mut user_guess = String::new();

    // Gets the imported io library, and we access its methods.
    io::stdin().read_line(&mut user_guess)
        .expect("Failed to read line.");

    // Output what the user put in.
    println!("Nice! You entered: {}", user_guess);
}
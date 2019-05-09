// import a specific package, in this case standard out.
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Command line output
    println!("Guess the number!!");
    print!("Enter your number here: ");

    // Declare the random number, range of 1 - 101
    // u32 - unsigned 32 bit number - CANNOT have negative numbers
    // i32 - signed 32 bit number - can have negative numbers
    let random_num: u32 = rand::thread_rng().gen_range(1, 101);

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

    // Parse the user guess as an integer.
    let user_guess: u32 = user_guess.trim().parse()
        .expect("Please enter a number!!");

    // Output what the user put in.
    println!("You entered: {}", user_guess);

    // Comparing the user's guess, to see if it's larger, smaller or correct.
    match user_guess.cmp(&random_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Correct!")
    };
}
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate random number with a generator local to the current thread and is seeded by the OS
    // The range is specified as 1..=100 which means 1-100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess...");

    // Declare var to store input in
    let mut guess = String::new();
    // Get the input and place it in the var
    io::stdin()
        .read_line(&mut guess)// Need to make the reference mutable as well, hence `&mut guess`
        .expect("Failed to read line!");    // Catch the Err variant of the Result Enum, or return the Ok value (n bytes of the user input)

    println!("You guessed: {guess}");
}
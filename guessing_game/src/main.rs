use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate random number with a generator local to the current thread and is seeded by the OS
    // The range is specified as 1..=100 which means 1-100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);  // By default an i32
    println!("The secret number is: {secret_number}");

    println!("Please input your guess...");

    // Declare var to store input in
    let mut guess = String::new();
    // Get the input and place it in the var
    io::stdin()
        .read_line(&mut guess)// Need to make the reference mutable as well, hence `&mut guess`
        .expect("Failed to read line!");    // Catch the Err variant of the Result Enum, or return the Ok value (n bytes of the user input)

    // Cast guess to an unsigned 32 bit int
    let guess: u32 = guess.trim().parse().expect("Not a number, please type a number!");

    println!("You guessed: {guess}");

    // Ordering is a type with Enums `Less`, `Greater` & `Equal`. guess is compared to secret_number here
    // Match checks which output cme from the compare, and executes on the match.
    // Matches have arms that rep[resent a pattern to match against, and the code that should be run if matched
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => prinln!("You win!"),
    }
}
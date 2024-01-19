use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess...");

    // Declare var to store input in
    let mut guess = String::new();
    // Get the input and place it in the var
    io::stdin()
        .read_line(&mut guess)// Need to make the reference mutable as well, hence `&mut guess`
        .expect("Failed to read line!");    // Catch the Err variant of the Result Enum, or return the Ok value (n bytes of the user input)

    println!("You guessed: {guess}");
}
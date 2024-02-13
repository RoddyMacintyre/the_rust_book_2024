fn main() {
    let number = 7;

    if number % 4 == 0 {
        println!("Divisible by 4");
    }
    else if number % 3 == 0 {
        println!("Divisible by 3");
    }
    else if number % 2 == 0 {
        println!("Divisible by 2")
    }
    else {
        println!("Number is not divisible by 4, 3, or 2.");
    }

    // Condition must be a bool. if number {} is not valid!
    if number != 0 {
        println!("Number was something other than zero");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition {5} else {6};     // Return types must match (in this case both are i32)
    println!("The value of the number is: {number}");
}



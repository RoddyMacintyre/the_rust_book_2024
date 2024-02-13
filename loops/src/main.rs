fn main() {
    // loop {
    //     println!("Again!");
    // }

    // Use an expression after break as an optional return value
    let mut counter = 0;

    // Entire loop block is a statement that assigns a value to result
    let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels
    // In nested loops, loop labels can be used to exit the outer loop based on a condition in the inner loop
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;     // Exits the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping through a collection with for
    // Below code is error-prone.
    // Also slow because the compiler adds a runtime check of array bounds
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // Better version:
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // For loop with ranges
    // Countdown example with a range & rev function
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

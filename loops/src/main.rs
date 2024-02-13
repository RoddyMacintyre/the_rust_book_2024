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
}

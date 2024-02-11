fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    // Statements & Expressions

    // *** Statements are instructions that do not return anything. Function definitions are also statements ***
    let y = 6;
    // Because they do not return values, cannot assign a let statement to another statement as follows:
    // let x = (let y = 6)?;
    // let doesn't return anything, so x cannot bind its variable to anything
    // This is in contrast to other languages like C and Ruby where you can form the expression x = y = 6
    // And both x and y will bind to 6

    // *** Expressions are statements with a return value ***
    // In other words, they will evaluate to a value (6 + 5 evaluates to 11)
    // Expressions can be part of statements (in let y = 6; 6 is an expression, as it evaluates to 6!)
    // Calling a function is an expression
    // Calling a macro is an expression
    // A new scope in curly brackets is an expression

    let y = {
        let x = 3;
        X + 1   // Expressions do not include ending semicolons, or they will be statements then and not return a value
    };
    println!("The value of y is: {y}");
}

// In fn sig must declare the type of the parameter
fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

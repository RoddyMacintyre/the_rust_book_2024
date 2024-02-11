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
        x + 1   // Expressions do not include ending semicolons, or they will be statements then and not return a value
    };
    println!("The value of y is: {y}");

    let five_return_value = five(); // use the return value of a function to initialize the variable, equivalent to let x = 5;
    println!("five() return value: {five_return_value}");

    let x = plus_one(five_return_value);
    println!("The value of x is: {x}");
}

// In fn sig must declare the type of the parameter
fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

// Functions with return values
// Return values are declared with the arrow ->
// Return values are synonymous with the last/final expression in the block of the body of a function
// You can return early by using the return keyword & specifying a value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1   // Do not place a semi-colon `;` here! Will return an error [E0308] "mismatched types"
            // We expect an i32, but statemenst evaluate to "()" (unit type), aka "Nothing"
}

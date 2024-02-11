fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');
}

// In fn sig must declare the type of the parameter
fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

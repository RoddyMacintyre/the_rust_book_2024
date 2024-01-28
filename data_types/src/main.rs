fn main() {
    // Floating point. IEEE-754 standard
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric operations
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Result: -1

    let remainder = 43 % 5;

    println!("Sum:\t{sum}\nDifference:\t{difference}\nProduct:\t{product}\n\
                Quotient:\t{quotient}\nTruncated:\t{truncated}\nRemainder:\t{remainder}\n");

    // Booleans
    let t = true;
    let f: bool = false;  // Explicit type annotation

    // The char type
    // Char literals are always in SINGLE QUOTES!
    // Rust char = 4 bytes & Unicode Scalar Value (can mean more than just ASCII)
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("True:\t{t}\nFalse:\t{f}\nC:\t{c}\nZ:\t{z}\nEMOJI:\t{heart_eyed_cat}\n");

    // COMPOUND TYPES
    // 2 types: TUPLE & ARRAY

    // TUPLES:
    // multi-types, fixed size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructure the tuple:
    let (x, y, z) = tup;
    println!("Tuple values:\t{x}, {y}, {z}\n");

    // Tuple index notation (why is there no standard?????)
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("Tuple values:\t{five_hundred}, {six_point_four}, {one}");

    // Emtpy tuple is call "unit". Every function that doesn;t return another type of value implicitly returns a "unit".

    // ARRAYS:
    // Same type, fixed size
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Collect data on stack instead of heap!

    // Example usage of fixed size container
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // Pre populate an array:
    let a = [3; 5];
    println!("Resulting array:\n\t{a:?}\n");

    // Accessing array data
    let first = a[0];
    let second = a[1];
    println!("First:\t{first}\nSecond:\t{second}\n");
}

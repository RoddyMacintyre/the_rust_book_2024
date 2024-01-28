fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants:
    // Not allowed to use `mut` keyword
    // Const data MUST be annotated to fix the type.
    // Const must be set only to a constant expression, not a runtime computed value
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The const value is: {THREE_HOURS_IN_SECONDS}");
    // Consts lifetime is the entire lifetime of its scope.

    // SHADOWING
    // To shadow, must use `let`
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Can temporarily change what the shadowed var means, to jump back to a previous meaning after scope is done.
    // Can also change the type
    let spaces = "     ";   // Won't work as let mut spaces; different types
    let spaces = spaces.len();
    println!("The length of spaces is: {spaces}");
}

// ========== Introduction ==========
/*
To understand when Structs might be useful, we'll write a program that calculates the area
of a rectangle. We'll start with single variables, and refactor them to structs along the way.

The program will take the width and height of a rectangle, specified in pixels, and calculate
the area.
 */

fn area(width: u32, height: u32) -> u32 {
    width * height
}

/*
The area function is supposed to calculate the area of a Rectangle, but it has 2 parameters.
Nowhere in the program is it clear that the parameters are related.
It would be more readable/manageable to group the 2 parameters together.
Let's refactor to a tuple.
 */

// ========== Refactoring with Tuples ==========

fn area_of_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    // Single variables
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Tuple version:
}

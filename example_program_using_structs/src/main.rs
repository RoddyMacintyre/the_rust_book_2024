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
/*
In one way the program is better by adding a bit of structure and passing one argument instead of 2.
It can however also be less clear because tuples don't have named elements, having us rely on indices.

Mixing height and width for calculating the area is no problem, but suppose you want to draw
the rectangle on screen. You would have to memorize which index represents which dimension.
We haven't conveyed the meaning of our data in our code, so it's easy to introduce subtle bugs and errors.
 */

fn area_of_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// ========== Refactoring with Structs ==========
/*
Structs add meaning to data by means of labels. We can transform the Tuple we're using into a
Struct with a name for the whole, as well as names for the parts.
 */

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_of_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/*
We have a Struct named Rectangle. Inside it, we define the fields as width and height (type u32).
In main, we created an instance of Rectangle with a width of 30 and height of 50.

the area func is now defined with one parameter, named rectangle and typed as an immutable borrowed struct of Rectangle.
We want to borrow rather than take ownership, so main can retain it and can continue using it (hence passing the reference &)

The area func accesses the width and height fields of the Rectangle instance (accessing fields in borrows does not move the field values).
The func sig now says exactly what we mean; calculate the area of a Rectangle, using width and height.
This conveys that width and height are related to each other, and gives descriptive power relative to the tuple.
 */


fn main() {
    // Single variables
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Tuple version:
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_tuple(rect1)
    );

    // Refactoring with Struct
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_of_struct(&rect2)
    )
}

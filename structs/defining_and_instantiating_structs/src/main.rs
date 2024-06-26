/*
Structs are similar to tuples: they both hold multiple related values.
With Structs, you name each data member.

Define a struct
Use they keyword `struct`, and name it.
Use curly brackets for the scope
Define names and types of the data (fields)
*/

// Example struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

/*
To use a struct after the definition, is to create an instance of it.
Need to specify values for each of the fields.

Create a struct
State the name of the struct
Add curly brackets for the value definitions
Provide key: value pairs for the field definitions

Struct definition is the general template, instances are concrete implementations of it

Can construct new instances as the last expression in a function body to implicitly return the new instance
Name the parameter names the same as the struct field names, but this can get tedious, so we can use short-hands
*/
fn build_user(email: String, username: String) {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// ========== Dot access ==========
// To get a specific value, use dot notation. If the instance is mutable, also use dot notation for mutations
// To mutate values, the entire instance must be mutable, certain mutable fields are not supported by Rust

// ========== Using the Field init shorthand ==========
/*
Can abbreviate initialization by leaving out the member names at initialization that are given as parameters in the factory
 */

fn build_user_abbreviated(email: String, username: String){
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// ========== Creating Instances from other Instances with Struct Update syntax ==========
/*
Sometimes you want to create new struct instances that differ only by a few Fields, and have the rest in common.
This can be achieved by the Update syntax.

..<struct> is the syntax, and comes after all other field assignments.
= is used as an assigment, because it moves the data.
CAN NO LONGER USE user1! The String in in the username field of user1 was moved to user2.
If we gave user2/2 new String values for both email and username, then user1 would still be valid, because nothing was moved
This is because active and sign_in_count types implement the Copy trait.
 */

// ========== Using Tuple Structs w/o named fields to create different types ==========
/*
Rust supports structs that look similar to tuples, called tuple structs.
They don't have Fieldnames, just Fields.

Handy when you want to define tuple types different from others and when fieldnames are redundant.

To define Tuple Structs: use struct keyword, followed by the types for the fields.
Each strcut you define is its own type, even though the fields in the structs may have the same types.

e.g. if a function asks a parameter of type Color, it won't accept the Point tuple struct.

Tuple structs can be destructured into individual pieces, and can use dot notation to access individual values/
 */

// Tuple Structs:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ========== Unit-Like Structs Without Any Fields ==========
/*
Can define structs without fields, called unit-like structs. Called so because they behave like (),
the unit type mentioned in The Tuple Type section.

Useful when you need to implement a trait on some type, but don't have any data to store in the type itself.
Traits are discussed in chapter 10.

Below an example of declaring & instantiating a unit struct named AlwaysEqual
 */
struct AlwaysEqual;

/*
To define AlwaysEqual, we use the struct keyword, the name we want, and then a semicolon. From there
we can get an instance  of AlwaysEqual in the subject variable.
Imagine later on AlwaysEqual is always equal to any instance of any other type, for testing purposes.
Now we don't need any data to implement that behavior.
 */

// ========== Ownership of Struct Data ==========
/*
In the user struct we used the owned String type rather than the &str string slice type.
This was done deliberately to have the struct instances each to own all of its data,
and for it to be valid for the lifetime of the instance.

It's possible for structs to store references to data owned by something else, but this requires lifetimes.
Lifetimes are discussed in chapter 10. They ensure the data referenced is valid for the lifetime of the struct.

Below an example of trying to store a reference in a struct without specifying the lifetime:
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_cout: 1,
    };
}

The compiler will complain about a lifetime specifier. It will suggest something along the lines of
struct User<'a> {
    username: &'a str,
    ...
 */

// ========== Borrowing Fields of a Struct ==========
/*
Similar to different tuple fields, Rust's borrow checker will track ownership permissions at both
the Struct level and field level. E.g. if we borrow a field x of a Point struct,
then both p and p.x temporarily lose their permissions (but not p.y).

So, be mindful which fields of a Struct are supposed to be borrowed with which permissions.
At the same time be aware of the borrow checker's limitations, since Rust assumes more borrowing
than is actually occurring.
 */

fn main() {
    // Instance of the struct example
    let mut user1 = User {
        email: String::from("someone@example.com"),     // On heap
        username: String::from("someusername123"),      // On heap
        active: true,
        sign_in_count: 1,
    };

    // Set a field by dot notation
    user1.email = String::from("anotheremail@example.com");     // On heap

    // Regular way of creating a new struct instance based on another one
    // The references are stored on the heap
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // With Update syntax
    // ..user1 says that the rest of the fields are set the same as user1
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple Structs
    // black and origin are instances of different tuple struct types.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    let subject = AlwaysEqual;

    // Borrowing Fields of a Struct
    struct Point {x: i32, y: i32}

    let mut p = Point {x: 0, y: 0};    // All permissions

    let x = &mut p.x;   // p & p.x lose their permissions, but p.y retains them
    *x += 1;    // Has RWO permissions on p & p.x

    // If we now try to use p while p.x is mutably borrowed, the compiler will reject the program.
    // The compiler will say something along the lines of:
    // error[E0502]: cannot borrow `p` as immutable because it is also borrowed as mutable
}

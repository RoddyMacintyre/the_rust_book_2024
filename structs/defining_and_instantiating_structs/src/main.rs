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
}

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
}

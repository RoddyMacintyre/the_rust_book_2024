/*
Several case studies of ownership errors and how to respond

Each case study is a function rejected by the compiler, and explanation, and a fix
The common theme is identifying safe/unsafe code

Rust will always reject unsafe programs, and sometimes might reject a safe one.
 */

// ========== Returning a reference to the stack ==========
// Data must always outlive its references
fn return_a_string() -> &string{
    let s = String::from("Hello world");
    &s
}

// The String lives in the scope of the function, and not outside it.
// So passing a reference to it as a return value, will break.
/*
4 ways to deal with this:
    - Move ownership out of the function by returning the value and not a reference

        fn return_a_string() -> String {
            let s = String::from("Hello world");
            s
        }

    - Return a string literal with unlimited lifetime (indicated by a 'static keyword

        fn return_a_string() -> &'static str {
            "Hello world"
        }

    - Defer borrow checking to runtime by using garbage collection with a reference-counted pointer

        use std::rc::Rc;
        fn return_a_string() -> Rc<String> {
            let s = Rc::new(String::from("Hello world"));
            Rc::clone(&s)
        }

    - Provide a slot to put the String, using a mutable reference

        fn return_a_string(output: &mut String) {
            output.replace_range(.., "Hello world");
        }
 */

// ========= Not enough permissions ==========
// Trying to mutate read-only data, or drop data behind a reference

fn stringify_name_with_title(name: &Vec<String>) -> String{
    // Name is an immutable reference
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

/*
Ways to deal with this:
    - Change to a mutable type
        BUT: functions should not mutate their inputs if the caller isn't expecting it
        (says stringify, but actually modifies the vector!)

    - Take ownership of the name by changing &Vec to Vec
        BUT: rare to take ownership of heap-owning data. The name variable becomes unusable

        fn stringify_name_with_title(mut name: Vec<String>) -> String {
            name.push(String::from("Esq."));
            let full = name.join(" ");
            full
        }
    - Clone the input name, so allowed to mutate a local copy.

        fn stringify_name_with_title(name: &Vec<String>) -> String {
            let mut full = name.join(" ");  // .join makes a copy of the data into the string full
            full.push_str(" Esq.");
            full
        }
 */

fn main() {
    println!("Hello, world!");
}

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


fn main() {
    println!("Hello, world!");
}

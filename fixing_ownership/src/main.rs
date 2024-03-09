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

// ========== Aliasing & mutating a Data Structure ==========
// Unsafe: using a reference to heap data that gets deallocated by another alis

fn add_big_strings(dst: &mut Vec<String>, src: &[String]){
    let largest: &String = dst.iter()
        .max_by_key(|s| s.len())
        .unwrap();

    for s in srd{
        if s.len() > largest.len(){     // Breaks after the first iteration with true
            dst.push(s.clone());
        }
    }
}

// let largest removes write permission to dst,
// but in the same scope the function attempts to push to dst and requires write permission
// dst.push could deallocate the contents of dst, invalidating the reference to largest

/*
Ways to deal with this:
All solutions: Shorten the lifetime of largest to not overlap with dst.push.

    - clone largest (performance hit)

        fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
            let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
            for s in src {
                if s.len() > largest.len() {
                    dst.push(s.clone());
                }
            }
        }

    - Perform all length comparisons first, then mutate dst (performance hit for allocating to vector to_add)

        fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
            let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
            let to_add: Vec<String> = src.iter()
                                        .filter(|s| s.len() > largest.len())
                                        .cloned()
                                        .collect();
            dst.extend(to_add);
        }

    - Copy out length of largest, since we don;t need the actual contents (most idiomatic and performant)
        LOOK AT WHAT YOU REALLY NEED!

        fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
            let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
            for s in src {
                if s.len() > largest_len {
                    dst.push(s.clone());
                }
            }
        }
 */

// ========== Copying vs. Moving out of a collection ==========
// Safe see main 1st 3 expressions
// With strings the necessary permissions are different than for ints
// Strings don't have the COPY TRAIT
// This can cause a DOUBLE-FREE, where 2 vars think they own a value, and at the first var drop the string is freed
// But when the second var drops, the string is attempted to be freed again

/*
Copying a string copies a pointer to heap data
Copying an i32 does not do this
(String does not implement copy trait, i32 does)

TO SUMMARIZE:
if a value does not own HEAP data, then it can be copied without a move
- i32 does not own heap data
- String does own heap data
- &String does not own heap data

AN EXCEPTION IS MUTABLE REFERENCES, E.G. &mut i32 is not a copyable type

Ways to deal with this:

- AVOID TAKING OWNERSHIP OF THE STRING & JUST USE IMMUTABLE REFERENCE
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];

- CLONE DATA IF YOU WANT OWNERSHIP OF THE STRING WHILE LEAVING VECTOR ALONE
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');

- USE A METHOD LIKE Vec::remove TO MOVE THE STRING OUT OF THE VECTOR
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
 */



fn main() {
    // Safe copying out of a collection
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];    // Read allowed
    let n: i32 = *n_ref;    // Dereference n_ref
}

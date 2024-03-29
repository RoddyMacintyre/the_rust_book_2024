// Variables live in the stack inside frames.
// A frame is a mapping from variables to values within a single scope (e.g. a function)
// For instance, there is a frame for main in which x is mapped to a value of true

// Frames are organized into a stack of currently called functions.
// After func returns, the frame is deallocated from the stack

// When an expression reads a variable, the value is copied from its slot into the stack frame

// ========== HEAP ==========
// When copying a lot of data, pointers can be used. Rust uses this as well.
// A pointer is a value that describes a location in memory (pointee)
// A common way to make a pointer is to allocate memory in the HEAP

// HEAP: Separate region in mem where data can live indefinitely, and is not tied to a specific stack frame
// BOX: Rust uses the concept of a box for putting data on the heap

// RUST does not permit manual memory management!
// Stack frames are automatically managed by Rust

// With BOX, how does Rust free this memory?
// A box's owner manages this deallocation
// If a var is bound to a box, when Rust deallocates the vsr's frame, Rust deallocates the box's heap memory

// So what if 2 vars hold a reference to a box?
// Rust uses ownership to resolve the issue of trying to release the heap box twice!

// ========== BOX ==========
// Boxes are used in Rust data structures like Vec, String, HashMap, etc.

// ========== MOVING ==========
// ! Variables cannot be used after being moved !
// Moves happen by default if there is no copy trait associated with the type!

// ========== CLONING ==========
// Cloning can avoid moves. Clone the var you pass as an arg to a func, and keep the original intact for later reference

// ########## REFERENCING ##########
// Referencing is a way to handle variables in scopes without moves. They are non-owning pointers
// Referencing is like borrowing

// ########## AVOIDING SIMUL ALIASING & MUTATION
// Pointers enable aliasing (accessing same data through different variables)
// Aliasing combined with mutations is dangerous and tricky
// Pointer Safety Principle: Data should never be aliased and mutated at the same time

// Data should never be aliased and mutated at the same time! Rust solves this with owner moves.
// Owned data can only be accessed by the owner.

// Rest is on the borrow checker. This ensures the safety of references (non-owning pointers), made to temporarily create aliases

// Avoiding simultaneous Aliasing & Mutation
// Aliasing & mutation together is dangerous for obvious reasons.
// - Deallocation of the aliased data leaves the other variable to point to deallocated memory
// - Mutation can leave runtime properties invalidated
// - Concurrently mutating the aliased data, causing data races, with nondeterministic behaviour

// ========== Borrow checker ==========
// Variables have 3 kinds of permissions:
//
// - Read: data can be copied to another location
// - Write: data can be mutated in-place
// - Own: data can be moved or dropped

// These permissions are not at runtime, but compile time
// Defaults to RO (read + own)
// "let mut" gives a variable write permission
// References can temporarily remove these permissions

// Permissions are defined on paths and not just variables.
// A path is anything on the left hand side of an assignment

fn permission_intro() {
    let mut v: Vec<i32> = vec![1, 2, 3];    // Permissions: RWO
    let num: &i32 = &v[2];                  // Permissions: v: R, num: RO, *num: R
    println!("Third element is {}", *num);  // Permissions: v: RWO, num: -, *num: -
    v.push(4);                         // Permissions: v: -
}

// The borrow checker finds Permission Violations
// Creating a reference cause data to be temporarily read-only, until reference is no longer used
// Vars cannot be mutated when a reference is in use!

// Mutable references provide unique and non-owning access to data
// Mutable references aka unique references
// Mutable references can temporarily be downgraded to read-only by doing a &*var expression

// e.g.
fn mutable_refs(){
    let mut v: Vec<i32> = vec![1, 2, 3];        // Permissions: v: RWO
    let num: &mut i32 = &mut v[2];              // Permissions: v: -, num,: RO, *um: RW; v lost all permissions
    *num += 1;                                  // Permissions:
    println!("Third element is {}", *num);      // Permissions: v: RWO, num: -, *num: -
    println!("Vector is now {:?}", v);          // Permissions: v: -
}

// Permissions are returned at the end of a reference's lifetime
// Lifetime is span of first initialized til last used

fn lifetime_example(){
    let mut x = 1;
    let y = &x;     // Lifetime of y begins (permissions: x: r)
    let z = *y;     // Lifetime of y ends (permissions: x: RWO)
    x += z;
}

// Variables can have different lifetimes in each branch of a conditional
fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];

    if c.is_ascii_lowercase(){
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

// Data must outlive all of its references
// 2 ways:
// Reference drops in scopes
// Flow permissions: expected when an expressions uses either an input reference, or returns an output reference
// Flow doesn't change throughout the body

// Lifetime parameters; input & output refs are treated differently than references within a func body
// Rust uses F permissions to check safety of in/out references

fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);      // These vsariables neither own the passed argument, nor the String
    // Therefor, upon exiting this function, nothing on the heap is deallocated, just the stack frame.
}

fn add_suffix(mut name: String) -> String {
    // Pointer to the String data from the arg is moved from the passed var to the local var name
    name.push_str(" Jr.");  // Resize the heap allocation.
    // Create new larger allocation, write new string into new allocation, & free original heap memory
    // Initial passed var now points to deallocated memory
    name    // Return statement without explicit keyword of semicolon!
}



fn boxed() {
    // A box is memory allocated on the heap for pointer use
    // First a OWNS the box
    let a = Box::new([0; 1_000_000]);
    // Then ownership is MOVED from a to b
    let b = a;
    // What happens when we exit here?
    // If a var owns a box, when Rust deallocates the var's frame, then Rust deallocates the box's heap memory
    // In this case on behalf of the variable b!
}

fn read(y: bool){
    if y {
        println!("y is true!");
    }
}


fn main() {
    let x = true;
    read(x);

    // Create, move, and mutate a String
    // first gets moved to name inside the add_suffix function, and then moved to full in the main function.
    // Eventually full owns the value
    let first = String::from("Ferris"); // Allocated to the heap
    let full = add_suffix(first);
    println!("{full}");

    // Referencing
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    greet(&m1, &m2);    // PAss references
    let s = format!("{} {}", m1, m2);
    println!("Formatted after greet, references are kept:\n\t{s}");

    // ========== DEREFERENCING ==========
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;    // *x reads the heap value, which is 1
    *x += 1;            // *x on the left side of an expression modiefies the heap value, so x points to 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack, and x points to a value in the heap
    let b: i32 = **r1;      // Therefor, need to dereference twice down the path to tget the heap value (2)

    let r2: &i32 = &*x;     // r2 points to the heap value directly, it's a reference to the dereferenced x
    let c: i32 = *r2;       // So only one dereference is needed to read it

    // Explicit conversions of references and pointers
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);  // Explicit dereference
    let x_abs2 = x.abs();       // Implicit dereferencing with dot notation
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);     // Explicit dereference twice
    let r_abs2 = r.abs();           // Implicit dereference twice
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);      // Explicit reference
    let s_len2 = s.len();           // Implicit reference
    assert_eq!(s_len1, s_len2);
    // Dot syntax is syntactic sugar for the function call syntax

    mutable_refs();
}

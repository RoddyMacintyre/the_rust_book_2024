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

fn boxed() {
    // A box is memory allocated on the heap for pointer use
    let a = Box::new([0; 1_000_000]);
    let b = a;
}

fn read(y: bool){
    if y {
        println!("y is true!");
    }
}


fn main() {
    let x = true;
    read(x);
}

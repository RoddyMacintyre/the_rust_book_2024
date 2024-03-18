/*

Slices let you reference a contiguous sequence subset of elements from a collection
It's a reference, so a non-owning pointer

The first_word function is tedious, because you have to keep track of lots of state when
trying to separate more words from the string.
For this reason, slices exist.

 */

// Func that takes a string of words separated by a space and returns the first word it finds
// Return the index of the end of the word
//  But, it returns a usize, which is not really meaningful outside of the context of the
// argument passed to first_word
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ========== STRING SLICES ==========
/*
Reference to a part of a string
String slices become str types

Slices are "fat" pointers: pointers with metadata
(pointer and len which define the underlined regions of the string on the heap)

Slices are references, so they change the permissions on referenced data
The original String loses write and own permissions when a slice is made and regains it after creation

let s = String::from("hello world");

let hello: &str = &s[0..5];     // hello
let world: &str = &s[6..11];    // world
let s2: &String = &s;
 */

// ========== RANGE SYNTAX ==========
/*
Can omit the first number in a slice if you start from 0
[0..2] = [..2]
The same counts for the end
[3..len] = [3..]
Entire string:
[..]

NOTE: In utf-8 multibyte chars some problems arise with regular slicing.
Will assume ASCII
 */

// =========== REWRITING first_word ==========
/*
String slices are expressed as &str
The following func returns a slice of the String passed in, which is a contextual return
It holds the starting point of the slice and the number of elements
 */

fn _first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ========== String literals are slices
/*
let s = "Hello, world!";
The type of s is &str; a slice pointing to that specific point in the binary
Also the reason that they are immutable
 */

fn main() {
    let mut s = String::from("Roddy Macintyre");    // s on the stack, "Roddy Macintyre" on the heap
    let index_found = first_word(&s);
    println!("{index_found}");
    s.clear();
    // From this point, index_found has no meaning

    let mut _s = String::from("Roddy Macintyre");
    let slice_found = _first_word(&_s);     // Removes write permission from _s
    println!("{slice_found}");
    _s.clear();
    // Below now gives error, and describes why the error occurs
    // If we have an immutable reference to something, we can;t also have a mutable reference to it
    // The print uses a slice, and the immutable reference needs to be active, but it's cleared!
    // Compile time dealing with this error!
    println!("{slice_found}");
}

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

fn main() {
    let mut s = String::from("Roddy Macintyre");    // s on the stack, "Roddy Macintyre" on the heap
    let index_found = first_word(&s);
    println!("{index_found}");
    s.clear();
    // From this point, index_found has no meaning
}

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

fn main() {
    let mut s = String::from("Roddy Macintyre");    // s on the stack, "Roddy Macintyre" on the heap
    let index_found = first_word(&s);
    println!("{index_found}");
    s.clear();
    // From this point, index_found has no meaning
}

// ========== Ownership vs garbage collection ==========
/*
Most programming langs use a garbage collector to manage memory (e.g. Python, JS, Java, Go).
GC works at runtime, adjacent to the app. It finds data that is no longer used,
and deallocates the unused memory for later use.

Key benefit of GC is that it avoids undefined behavior, and avoids need for complex typing system
to check for undefined behavior.

Key drawbacks are performance (frequent small overheads or infrequent large overheads).
A less obvious drawback is that GC can be unpredictable.
Take for example this Python class that represents a mutable list of words:

class Document:
    def __init__(self, words: List[str]):
        """Create a new document"""
        self.words = words

    def add_word(self, word: str):
        """Add a word to the document"""
        self.words.append(word)

    def get_words(self) -> List[str]:
        """Get a list of all the words in the document"""
        return self.words

Now we instantiate a Document d, copy it into d2 and then mutate d2.

words = ["Hello"]
d = Document(words)

d2 = Document(d.get_words())
d2.add_word("world")

- When is the list deallocated?
The program has 3 pointers to the same list. Deallocation only happens when all 3 var are out of scope.

- What are the contents of d2?
d2 is a pointer to d, which has a pointer to words. so d contains ["Hello", "world"].
d.words() returns a mutable reference to d. This is implicit mutable reference passing.

This problem not only happens in Python, but also C#, JAva, JS, etc.
Most programming languages have a concept of pointers, but the languages expose this in different ways.
GC makes it difficult to track pointers (e.g. wasn't clear d.get_words() produces a pointer to data in d.

Rust's ownership model puts pointers front-and-center:
 */

type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    // Consumes ownership of the input vector "words"; Document owns word.
    // vector word is deallocated when this goes out of scope.
    words
}

fn add_word(this: &mut Document, word: String) {
    // Requires a mutable reference &mut Document to be able to mutate the document.
    // Also consumes ownership of the input "word"; no one else can mutate the individual words of the document.
    this.push(word)
}

fn get_words(this: &Document) -> &[String] {
    // Returns explicit immutable reference to strings within the document.
    // Only way to create new document from this is with a deep copy (not a reference)
    this.as_slice()
}

// ========== The concepts of Ownership ==========
/*
Ownership at runtime
- Allocates local variables in stack frames, which are allocated when a function is called and deallocated when the call ends
- Local vars can hold either data, or pointers
- Pointers can be created either through boxes (pointers of heap data) or references (non-owning pointers)
 */

/*
Ownership at compile-time
Rust tracks RWO permissions on each variable, and requires appropriate permissions for operations.
e.g. let mut misses the W permission and cannot be mutated.

Permissions can be changed when a variable is moved or borrowed.

MOVE
- move with non-copyable type requires RO permissions, and the move eliminates all permissions on the var
That rule prevents the use of moved variables.

The following demonstrates this:
fn main() {
    let s = String::from("Hello world");    // RO
    consume_a_string(s);                    // No RO
    println!("{s}");                        // No read after moving s, so cannot print it
}

BORROW
Borrowing = creating a reference, temporarily removes some permissions.
- Immutable borrow creates an immutable reference, and disables borrowed data from mutations and moves

The following demonstrates this:

Can print immutable reference
let mut s = String::from("Hello");      // RWO
let s_ref = &s;                         // RO (s becomes R, *s = R)
println!("{s_ref}");    // Prints

Cannot mutate immutable reference
let mut s = String::from("Hello");
let s_ref = &s;
s_ref.push_str(" world");   // Not OK

Cannot mutate the immutably borrowed data
s.push_str(" world");       // Not OK

Cannot move data out of the reference
let s2 = *s_ref;            // Not OK

A mutable borrow creates a mutable reference, which disables the borrowed data from being read, written, or moved.
This is to prevent the var from changing when borrowed to mutate. The intention of the borrow is clear, so we give owner the right of way.

Can mutate the mutable reference
s_ref.push_str(" world");

Cannot access the mutably borrowed data
println!("{s}");            // Not OK
 */

// ========== Connecting Ownership between compile-time and runtime
/*
Rust's permissions are designed to prevent undefined behavior.
- use-after-free = when freed memory is read or written => immutable borrows remove the W to prevent them
- Double-free = memory is freed twice => dereferences of references to non-copyable data don't have the O to avoid these
 */


fn main() {
    // Deep copy a Document
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    // The modification to d2 does not affect d
    assert!(!get_words(&d).contains(&"world".into()))
}
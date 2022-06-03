fn main() {
    // because the memory used by a String is unknown during compile time, it has to be allocated
    // on heap, and push a pointer on to the stack
    let s1 = String::from("hello");
    let s2 = s1.clone(); // copies both pointer on stack and data heap
    let s = "hello world";

    println!("{}", s1);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

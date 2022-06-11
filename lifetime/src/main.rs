use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_in_function_signature() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("yaya {}", result);
}

// hold references in structs
// this means that the lifetime of the struct
// cannot outlive the reference it holds
struct Yaya<'a> {
    reference: &'a str,
}

impl<'a> Yaya<'a> {
    fn level(&self) -> i32 {
        // no need lifetime annotation because an owned type is returned
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // third elision rule
        // if a method has input &self
        // then the return also get lifetime of &self
        println!("Attention please: {}", announcement);
        self.reference
    }
}

fn lifetime_in_struct() {
    let novel = String::from("from russia with love. putin is big gae");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = Yaya {
        reference: first_sentence,
    };
    println!("{}", i.reference);
}

fn lifetime_in_methods() {
    let novel = String::from("from russia with love. putin is big gae");
    let mut sentences = novel.split('.');
    let first_sentence = sentences.next().expect("Could not find a '.'");
    let second_sentence = sentences.next().expect("Could not find a '.'");
    let i = Yaya {
        reference: first_sentence,
    };

    let x = i.level();
    println!("{}", x);
    i.announce_and_return_part(second_sentence);
}

// generic type parameters, trait bounds, and lifetimes all in one function
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    lifetime_in_function_signature();
    lifetime_in_struct();
    lifetime_in_methods();

    // all string literals have the 'static lifetime
    // which can live for the entire duration of the program
    let s: &'static str = "yaya";
}

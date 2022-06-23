use std::{
    fmt,
    ops::{Deref, DerefMut},
};

pub fn newtype_pattern() {
    let mut w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("w = {}", w);

    // deref allows the wrapper to behave just like Vec<String>
    w.push(String::from("yaya"));
    println!("w = {}", w);
}

// the orphan rule prevents us from implementing external traits on external types
// we can get around this by creating a wrapper on the external type,
// making it local to our crate
// the wrapper uses the struct-tuple with only one item
// there is no run time penalty as the wrapper type is elided at compile time.

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// can implement the deref trait,
// if we want the new type to have every method the inner type has
impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

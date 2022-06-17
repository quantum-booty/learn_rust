use std::ops::{Deref, DerefMut};

// Box<T> is a tuple struct with one element
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("{}", name);
}

pub fn deref() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // *y on a Box object calls this code under the hood
    // deref gets a regular reference that the compiler knows how to * deref
    let j: &i32 = y.deref();
    let _k: i32 = *j;

    // deref coercion
    // converts &MyBox<String> into &String by calling deref
    let z = MyBox::new(String::from("yaya"));
    hello(&z);
    // works like this under the hood
    hello(z.deref());

    // DerefMut allows us to return a mutable reference with deref_mut
    let mut l = MyBox::new(2);
    *l += 1;

    // deref coercion rules
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>

    // its possible to convert a mutable reference to an immutable rereference because of the
    // borrowing rules, if you have a mutable referenc, that mutable reference must be the only
    // reference to that data
    // whereas the reverse is not true
}

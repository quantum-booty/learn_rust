use crate::boxing::List::{Cons, Nil};

pub fn boxing() {
    // uses of Box
    // When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    // Box::new(x) moves x, unless x implements Copy trait, e.g.
    // let x = 5;
    // let y = Box::new(x); // x is copied
    //
    // let j: Vec<i32> = Vec::new();
    // let k = Box::new(j); // j is moved


    // stores an i32 value on the heap, and a pointer on the stack
    let b = Box::new(5);
    println!("{}", b);

    // a recursive data type does not have a known size at compile time
    // cons list
    // (d, (c, (b, a))), (c, (b, a)), (b, a), (a, nil), nil

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // because Box<T> is a pointer, the compiler knows how much Box<T> needs
    // the Cons variant would need the size of i32 + size of Box<List> which is the size of a pointer
    //
    // Box<T> type is a smart pointer because it implements the Deref and Drop trait
    // Deref:
    // Allows Box<T> values to be treated like references.
    // Drop:
    // When a Box<T> value gues out of scope, the heap data that the box is
    // pointing to is cleaned up.
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

mod boxing;
mod combining_Rc_and_RefCell;
mod deref;
mod drop;
mod interior_mutability;
mod reference_counted;

fn main() {
    // A smart pointer is something that implements the Deref and Drop traits
    //
    // Deref: allows an instance of the smart pointer struct to behave like a
    // reference so you can write code that works with either references or smart pointers.
    //
    // Drop: allows to customise the code to run when an instance of the smart
    // pointer goes out of scope
    boxing::boxing();
    deref::deref();
    drop::drop();
    reference_counted::reference_counted();
    interior_mutability::interior_mutability();
    combining_Rc_and_RefCell::combining_Rc_and_RefCell();

    //                 ownership | single threaded only | allow mutable borrow | borrow rule enforced at
    // Rc              multiple  | true                 | false                | compile time
    // Box             single    | false                | true                 | compile time
    // RefCell         single    | true                 | interior mutable     | run time
    // Rc<RefCell<T>>  multiple  | true                 | interior mutable     | compile time + run time

    // interior mutability
    // can mutate the value inside RefCell<T> even when the RefCell<T> is immutable
}

mod boxing;
mod deref;
mod drop;
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
}

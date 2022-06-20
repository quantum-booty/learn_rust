use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn shared_state_concurrency() {
    // we need to move Mutex into each spawned thread
    // to enable multiple ownership, we need something like Rc
    // but safe to share across threads
    // We can use Arc, which stands for Atomic reference counted pointer
    // Arc has the same api as Rc
    // The reason why Rc aren't thread safe by default is because thread safety comes with a
    // performance penalty that you only want to pay when you really need to.

    // the counter is an immutable reference
    // however we could get a mutable reference to the value inside it
    // this is because Mutex provides interior mutability, just like RefCell
    // Mutex comes with the risk of creating deadlocks, when an operation needs to lock two
    // resources and two threads have each acquired one of the locks, causing them to wait for each
    // other forever.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

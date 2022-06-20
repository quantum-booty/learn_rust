mod spawning_threads;
mod share_memory_by_communicating_using_channel;
mod shared_state_concurrency;

fn main() {
    // concurrent programming => different part of a program execute independently
    // parallel programming => different parts of a program execute at the same time

    // an executed program's code is run in a process, the OS manages multiple processes at once
    // in a program, can have independent parts that run simultaneously in different threads

    spawning_threads::spawning_threads();
    share_memory_by_communicating_using_channel::share_memory_by_communicating_using_channel();
    shared_state_concurrency::shared_state_concurrency();

    // Send trait
    // The Send trait indicates that ownership of values of the type implementing Send can be
    // transferred between threads
    // almost all primitive types are Send, aside from raw pointers
    // Rc does not implement Send where as Arc does
    // This is so that the user can choose to op-in for performance penalty that comes with thread
    // safety

    // Sync trait
    // The Sync trait indicates that it is safe for the type implementing Sync to be referenced
    // from multiple threads.
    // In other words, any type T is Sync if &T (an immutable reference to T) is Send,
    // meaning the reference can be sent safely to another thread.
    // Similar to Send, primitive types are Sync, and types composed entirely of types that are
    // Sync are also Sync.
    // Rc<T> is not Sync and RefCell<T> and the family of related Cell<T> types are not Sync. The
    // implementation of borrow checking that RefCell<T> does at runtime is not thread-safe.
    //
    // The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads.
}

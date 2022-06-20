use std::{sync::mpsc, thread, time::Duration};

pub fn share_memory_by_communicating_using_channel() {
    // a channel has two parts, a transmitter and a receiver
    // we can create one thread to generate the values, and send them down a channel
    // and another thread that will receive the values and print them out

    // mpsc stands for multiple producer, single consumer FIFO queue
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    // move the transmitting end into a spawned spread
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        // in real application, instead of using unwrap, the Err should be handled properly
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // we can have multiple producers
    thread::spawn(move || {
        let vals = vec![
            String::from("shine"),
            String::from("on"),
            String::from("you"),
            String::from("crazy"),
            String::from("diamonds"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv blocks the main thread's execution and wait until a value is sent down the channel
    // it returns Result<T, E>
    // when the sending end of the channel closes, recv will return Err to signal that no more
    // values will be coming
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    // try_recv doesn't block, useful if this thread has other work to do while waiting for
    // messages

    // can also receive in a loop
    for received in rx {
        println!("Got: {}", received);
    }
}

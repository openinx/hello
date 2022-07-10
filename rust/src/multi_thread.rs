#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc, Mutex};
    use std::{thread, time::Duration};

    #[test]
    pub fn basic_print() {
        let (tx, rx) = mpsc::channel();

        let mut x = 0;
        let a = thread::spawn(move || {
            // Start the for loop.
            for _ in 0..20 {
                x += 1;
                thread::sleep(Duration::from_millis(200));
                println!("The current x is: {}", x);
            }

            // Send the incremented value to the writer of channel.
            tx.send(x).unwrap();
        });

        a.join().expect("Unexpected error when joining the thread");

        // Read the result value from the reader of channel.
        assert_eq!(Ok(20), rx.recv());
    }

    #[test]
    pub fn multi_thread_inc_same_value() {
        // Refer to the examples from here:
        // https://doc.rust-lang.org/book/ch16-03-shared-state.html

        // Define a i32 which is wrapped by `Mutex` and `Arc`.
        //
        // The `Arc` is used for sharing between different threads, as each thread
        // need a separate ownership to mut the `counter` ( with the keyword `move`)
        //
        // The `Mutex` is used to lock & unlock between threads. Before mute the counter
        // we will need to call `lock` first, and then increment the value, finally the
        // `MutexGuard` will unlock the mutex implicitly (see the `drop` trait for
        // `MutexGuard`).
        let count = Arc::new(Mutex::new(0));

        // Define an array list to collect all the spawned threads.
        let mut threads = vec![];
        for id in 0..10 {
            // Clone the `Arc` counter so that every thread can take its ownership.
            let clone_count = count.clone();
            let t = thread::spawn(move || {
                for _ in 0..10 {
                    {
                        // !!! BE CAREFUL !!!
                        // Move the `MutexGuard` into a separate block to release
                        // the mutex as soon as possible, for not including the thread
                        // sleep to block other threads.
                        let mut num = clone_count.lock().unwrap();
                        *num += 1;

                        println!("Thread {} increment the counter to {}", id, *num);
                    }

                    thread::sleep(Duration::from_millis(100));
                }
            });
            threads.push(t);
        }

        for t in threads {
            t.join().unwrap();
        }

        assert_eq!(100, *count.lock().unwrap());
    }
}

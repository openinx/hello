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
                        // sleep into lock context to block other threads.
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

    /// Let's create two threads and one print even numbers and another one print old
    /// numbers. Finally make them sychronize with each other to print the whole 0..n
    /// sequence.
    #[test]
    pub fn even_old_game() {
        enum State {
            Even,
            Old,
        }

        let result = Arc::new(Mutex::new(Vec::new()));

        let mux = Arc::new(Mutex::new(State::Even));

        let e_mux = mux.clone();
        let e_result = result.clone();
        let even = thread::spawn(move || {
            for num in (0..=100).step_by(2) {
                loop {
                    let mut state = e_mux.lock().unwrap();
                    if matches!(*state, State::Even) {
                        println!("Even thread print {}", num);
                        (*e_result.lock().unwrap()).push(num);
                        *state = State::Old;
                        break;
                    }
                }
            }
        });

        let o_mux = mux.clone();
        let o_result = result.clone();
        let old = thread::spawn(move || {
            for num in (1..=100).step_by(2) {
                loop {
                    let mut state = o_mux.lock().unwrap();
                    if matches!(*state, State::Old) {
                        println!("Old thread print {}", num);
                        (*o_result.lock().unwrap()).push(num);
                        *state = State::Even;
                        break;
                    }
                }
            }
        });

        even.join().unwrap();
        old.join().unwrap();

        assert_eq!(*result.lock().unwrap(), Vec::from_iter(0..=100));
    }
}

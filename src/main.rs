use std::{io, vec};

mod sum;
mod prime;
mod qsort;
mod msort;
mod heapsort;
mod linked_list_v1;
mod linked_list_v2;
mod linked_list_v3;
mod rust_star;
mod deref_example;

fn main() {
    println!("Hello, world!");

    println!("Sum of 2 + 3 is: {}" , sum::sum(2, 3));
    println!("23 is primary ? answer: {}", prime::is_prime(23));

    let mut vec = vec![2, 4, 5, 3];
    qsort::qsort(&mut vec);
    println!("Quicksort result is: {:?}", vec);

    vec = vec![3, 2, 1];
    msort::merge_sort(&mut vec);
    println!("Mergesort result is: {:?}", vec);

    vec = vec![3, 2, 1];
    heapsort::sort(&mut vec);
    println!("Heap sort result is: {:?}", vec);

    let mut list1 = linked_list_v1::List::new();
    list1.push(1);
    println!("LinkedList v1 pop() is: {}", list1.pop().unwrap());

    let mut list2 = linked_list_v2::List::new();
    list2.push(1);
    println!("LinkedList v2 peek() is: {}", list2.peek().unwrap());
    println!("LinkedList v2 pop() is: {}", list2.pop().unwrap());
    println!("LinkedList v2 is_empty() is: {}", list2.is_empty());

    rust_star::test_start();


    // Prompt
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    
    println!("You guessed: {}", guess);
}

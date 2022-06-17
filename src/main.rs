use std::{io, vec};

mod avl_tree;
mod cmp;
mod deref_example;
mod double_linked_list_v1;
mod double_linked_list_v2;
mod double_linked_list_v3;
mod hash_table;
mod hash_table_v2;
mod heapsort;
mod kmp;
mod linked_list_unsafe;
mod linked_list_v1;
mod linked_list_v2;
mod linked_list_v3;
mod merge_linkedlist;
mod msort;
mod prime;
mod qsort;
mod ref_cell;
mod rust_star;
mod select_sort;
mod simple_tree;
mod skiplist;
mod sum;
mod unsafe_dequeue;

fn main() {
    println!("Hello, world!");

    println!("Sum of 2 + 3 is: {}", sum::sum(2, 3));
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

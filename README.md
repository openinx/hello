## Rust Algorithm Practice.

### Quick Start

Follow the below command to verify the unit tests:

```bash
git clone git@github.com:openinx/hello_world.git
cd hello_world
cargo test --release
```

### Rust Basic

* [A + B](./src/sum.rs): Rust simple A + B.
* [Reference and Dereference](./src/ref_deref.rs): A simple example to show semantic of Reference and Dereference.
* [Deref Trait](./src/deref_trait.rs): A simple example to demonstrate `Deref` trait.
* [Check Prime](./src/prime.rs): Simple rust code to check whether it's a prime or not.
* [Compare And Ord](./src/cmp.rs): How to define a comparator for a customized struct or type.
* [RefCell](./src/ref_cell.rs): Simple example to demonstrate the [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html).

### List, Stack and Queue

__LinkedList__
* [Safe Linked List V1](./src/linked_list_v1.rs): A safe linked list implemented by Rust [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) and [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html). Refer to the [A Bad Stack](https://rust-unofficial.github.io/too-many-lists/first-final.html).
* [Safe Linked List V2](./src/linked_list_v2.rs): A safe linked list implemented by Rust [Option](https://doc.rust-lang.org/std/option/) and [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html). Refer to the [An Ok Stack](https://rust-unofficial.github.io/too-many-lists/second-final.html)
* [Safe Linked List V3](./src/linked_list_v3.rs): A safe linked list implemented by Rust [Option](https://doc.rust-lang.org/std/option/) and [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
* [Linked List implement by unsafe approach](./src/linked_list_unsafe.rs): A linked list implemented by unsafe approach. Refer to [An Ok Unsafe Singly-Linked Queue](https://rust-unofficial.github.io/too-many-lists/fifth.html#an-ok-unsafe-singly-linked-queue)
* [Merge Two Sorted Linked List](./src/merge_linkedlist.rs): Merge two sorted linked list in Rust.

__Double Linked List__
* [Safe Double Linked List V1](./src/double_linked_list_v1.rs): A safe double linked list implemented by `Option`, `Rc`, `RefCell`. Refer to [A Bad but Safe Doubly-Linked Deque](https://rust-unofficial.github.io/too-many-lists/fourth.html).
* [Safe Double Linked List V2](./src/double_linked_list_v2.rs): Another safe double linked list implementation, still use `Option`, `Rc` and `RefCell`.
* [A Simple Unsafe Double Linked List](./src/double_linked_list_v3.rs): A simple unsafe double linked list implementation.
* [A production-ready Unsafe Double Linked List](./src/double_linked_list_unsafe.rs): A production ready unsafe double linked list implementation. Refer to [A Production-Quality Unsafe Doubly-Linked Deque](https://rust-unofficial.github.io/too-many-lists/sixth.html).

### Sort

* [Selection Sort](./src/select_sort.rs): Refer to [Selection Sort](https://en.wikipedia.org/wiki/Selection_sort) in wikipedia.
* [Quick Sort](./src/qsort.rs): Refer to [Quick Sort](https://en.wikipedia.org/wiki/Quicksort) in wikipedia.
* [Heap Sort](./src/hash_table_v2.rs): Refer to [heap sort](https://en.wikipedia.org/wiki/Heapsort) in wikipedia.
* [Merge Sort](./src/msort.rs): Refer to [merge sort](https://en.wikipedia.org/wiki/Merge_sort) in wikipedia.

### Search

* [Safe HashTable V1](./src/hash_table_v1.rs): Use the vector to resolve hash conflicts for the given bucket.
* [Safe HashTable V2](./src/hash_table_v2.rs): Use linked list to resolve hash conflicts for the given bucket.
* [Safe Basic Binary Tree](./src/simple_tree.rs): A basic [binary search tree](https://en.wikipedia.org/wiki/Binary_search_tree) which supports binary search, insert, delete, visiting precursor, visiting succesor, finding max, finding min etc.
* [Safe AVL Tree](./src/avl_tree.rs): Safe [self-balancing binary search tree](https://en.wikipedia.org/wiki/AVL_tree).
* [Safe Skip List](./src/skiplist.rs): The hard and safe way to implement Safe [Skip List](https://en.wikipedia.org/wiki/Skip_list). Since each node in SkipList will be referenced by both vertical linked list and horizontal linked list, so the `Option<Rc<RefCell<..>>>` is required.
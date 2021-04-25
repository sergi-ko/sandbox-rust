#![allow(unused)]
pub fn test_binary_heap() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(vec![1, 2, 20, 3, 4, 5, 6, 7]);

    // Will print in some order
    while heap.len() > 0 {
        println!("{}", heap.pop().unwrap());
    }
}

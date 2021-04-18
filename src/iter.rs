use std::iter::FromIterator;
use std::time::Instant;

/// Notes/TODO:
///     - run in --release
///     - try https://crates.io/crates/cargo-criterion
///     - try https://doc.rust-lang.org/cargo/commands/cargo-bench.html
///     - try https://doc.rust-lang.org/std/hint/fn.black_box.html
///     - assembly can be checked here: https://godbolt.org/z/svjjfoxjW

pub struct VecStruct {
    v: Vec<u128>,
}

// pub fn struct_iter(vstruct: &VecStruct) {
//     for i in vstruct.v {
//         println!("{}", i);
//     }
// }
//
// error[E0507]: cannot move out of `vstruct.v` which is behind a shared reference
//  --> src/iter.rs:6:14
//   |
// 6 |     for i in vstruct.v {
//   |              ^^^^^^^^^
//  |              |
//  |              move occurs because `vstruct.v` has type `Vec<u64>`, which does not implement the `Copy` trait
//  |              help: consider iterating over a slice of the `Vec<_>`'s content: `&vstruct.v`
//
// error: aborting due to previous error; 1 warning emitted
//
// For more information about this error, try `rustc --explain E0507`.

pub fn struct_iter_iter(vstruct: &VecStruct) -> u128 {
    let mut k = 0;
    for i in vstruct.v.iter() {
        k = *i;
    }
    k
}

pub fn struct_iter_clone(vstruct: &VecStruct) -> u128 {
    let mut k = 0;
    for i in vstruct.v.clone() {
        k = i;
    }
    k
}

pub fn struct_iter_slice(vstruct: &VecStruct) -> u128 {
    let mut k = 0;
    for i in &vstruct.v {
        k = *i;
    }
    k
}

pub fn struct_iter_field(v: &Vec<u128>) -> u128 {
    let mut k = 0;
    for i in v {
        k = *i;
    }
    k
}

// Result on my machine
// struct_iter_iter        2.071793839s
// struct_iter_clone       1.844594912s
// struct_iter_slice       2.053948301s
// struct_iter_field       2.064109265s
//
pub fn test_performance() {
    println!("Testing performance");

    let vstruct = VecStruct {
        v: Vec::from_iter(std::iter::repeat(5).take(999_999)),
    };

    let start = Instant::now();
    for i in (0..999_999).enumerate() {
        struct_iter_iter(&vstruct);
    }

    let duration = start.elapsed();
    println!("struct_iter_iter\t{:?}", duration);

    let start = Instant::now();
    for i in (0..999_999).enumerate() {
        struct_iter_clone(&vstruct);
    }

    let duration = start.elapsed();
    println!("struct_iter_clone\t{:?}", duration);

    let start = Instant::now();
    for i in (0..999_999).enumerate() {
        struct_iter_slice(&vstruct);
    }

    let duration = start.elapsed();
    println!("struct_iter_slice\t{:?}", duration);

    let start = Instant::now();
    for i in (0..999_999).enumerate() {
        struct_iter_field(&vstruct.v);
    }

    let duration = start.elapsed();
    println!("struct_iter_field\t{:?}", duration);
}

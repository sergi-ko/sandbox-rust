use std::io;
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

pub fn struct_iter_iter(n: u128, vstruct: &VecStruct) -> u128 {
    let mut k = 999_999_999;
    for i in vstruct.v.iter() {
        k ^= n + i;
    }
    k
}

pub fn struct_iter_clone(n: u128, vstruct: &VecStruct) -> u128 {
    let mut k = 999_999_999;
    for i in vstruct.v.clone() {
        k ^= n + i;
    }
    k
}

pub fn struct_iter_slice(n: u128, vstruct: &VecStruct) -> u128 {
    let mut k = 999_999_999;
    for i in &vstruct.v {
        k ^= n + i;
    }
    k
}

pub fn struct_iter_field(n: u128, v: &Vec<u128>) -> u128 {
    let mut k = 999_999_999;
    for i in v {
        k ^= n + i;
    }
    k
}

// Result on my machine(--release, ITERATIONS=999)
// struct_iter_iter	1.053240662s, 999999992
// struct_iter_clone	2.773735007s 999999998
// struct_iter_slice	1.01796874s 999999992
// struct_iter_field	1.031684125s 999999998
//
// (debug, ITERATIONS=9)
//
pub fn test_performance() {
    const ITERATIONS: u128 = 999;
    let mut input = String::new();
    println!("Testing performance, provide integer number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Please provide integer");
    let input: u128 = match input.trim().parse() {
        Ok(n) => n,
        Err(e) => panic!(e),
    };

    let vstruct = VecStruct {
        v: Vec::from_iter(std::iter::repeat(5).take(999_999)),
    };

    let mut r = 1;
    let start = Instant::now();
    for i in (0..ITERATIONS).enumerate() {
        r ^= struct_iter_iter(input + i.1, &vstruct);
    }

    let duration = start.elapsed();
    println!("struct_iter_iter\t{:?}, {:?}", duration, r);

    let start = Instant::now();
    for i in (0..ITERATIONS).enumerate() {
        r ^= struct_iter_clone(input + i.1, &vstruct);
    }

    let duration = start.elapsed();
    println!("struct_iter_clone\t{:?} {:?}", duration, r);

    let start = Instant::now();
    for i in (0..ITERATIONS).enumerate() {
        r ^= struct_iter_slice(input + i.1, &vstruct);
    }

    let duration = start.elapsed();
    println!("struct_iter_slice\t{:?} {:?}", duration, r);

    let start = Instant::now();
    for i in (0..ITERATIONS).enumerate() {
        r ^= struct_iter_field(input + i.1, &vstruct.v);
    }

    let duration = start.elapsed();
    println!("struct_iter_field\t{:?} {:?}", duration, r);
}

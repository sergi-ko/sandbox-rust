pub mod binary_heap;
pub mod deadlock;
pub mod enums;
pub mod iter;
pub mod refcounts;
pub mod test;

use crate::iter::test_performance;
use crate::test::test1;
use crate::test::test2;

//use crate::deadlock::do_deadlock;

use crate::refcounts::do_refcount;

use crate::enums::enum_and_map;

use crate::binary_heap::test_binary_heap;

fn main() {
    let s = test1();
    println!("{}", s);

    println!("{}", test2());

    //    println!("{}", do_deadlock());

    //do_refcount();
    //   test_performance();
    //enum_and_map();

    test_binary_heap();
}

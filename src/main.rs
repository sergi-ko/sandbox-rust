pub mod deadlock;
pub mod refcounts;
pub mod test;

use crate::test::test1;
use crate::test::test2;

//use crate::deadlock::do_deadlock;

use crate::refcounts::do_refcount;

fn main() {
    let s = test1();
    println!("{}", s);

    println!("{}", test2());

    //    println!("{}", do_deadlock());

    do_refcount();
}

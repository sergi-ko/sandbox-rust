pub mod deadlock;
pub mod test;

use crate::test::test1;
use crate::test::test2;

use crate::deadlock::do_deadlock;

fn main() {
    let s = test1();
    println!("{}", s);

    println!("{}", test2());

    println!("{}", do_deadlock());
}

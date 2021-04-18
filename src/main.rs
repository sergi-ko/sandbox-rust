pub mod test;

use crate::test::test1;
use crate::test::test2;

fn main() {
    let s = test1();
    println!("{}", s);

    println!("{}", test2());
}

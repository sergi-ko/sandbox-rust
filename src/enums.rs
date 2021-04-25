use std::cmp::Ordering;
use std::collections::BTreeMap;

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resolution {
    Minute1 = 1,
    Minute3 = 3,

    Hour1 = 60,
    Hour2 = 120,

    Day1 = 1440,
}

pub fn enum_and_map() {
    let r = Resolution::Minute1;
    let mut m = BTreeMap::new();
    m.insert(r, 100);
    m.insert(r, 200);
    m.insert(Resolution::Day1, 100);
    m.insert(Resolution::Hour1, 1020);

    println!("{:?}", m);
    println!("{:?}", r as u16);

    if let Some(x) = m.values_mut().last() {
        println!("x={}", x);
    } else {
        println!("no x");
    }
    println!("end");
}

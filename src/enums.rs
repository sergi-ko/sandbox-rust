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
/*
impl Ord for Resolution {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.value, &self.name).cmp(&(other.value, &other.name))
    }
}*/

pub fn enum_and_map() {
    let r = Resolution::Minute1;
    let mut m = BTreeMap::new();
    m.insert(r, 100);
    m.insert(r, 200);
    m.insert(Resolution::Day1, 100);
    m.insert(Resolution::Hour1, 1020);

    println!("{:?}", m);
}

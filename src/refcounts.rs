use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum MyTree {
    Leaf(u8, Rc<MyTree>, Rc<MyTree>),
    Nil,
}

impl MyTree {
    fn left(&self) -> &MyTree {
        if let MyTree::Leaf(val, l, r) = self {
            //Rc::try_unwrap(*l).unwrap()
            l
        } else {
            &MyTree::Nil
        }
    }
    fn right(&self) -> &MyTree {
        if let MyTree::Leaf(val, l, r) = self {
            //Rc::try_unwrap(*l).unwrap()
            r
        } else {
            &MyTree::Nil
        }
    }
    fn add_left(&mut self, leaf: MyTree) {
        match self {
            MyTree::Leaf(val, l, r) => *l = Rc::new(leaf),
            MyTree::Nil => (),
        }
    }
    fn add_right(&mut self, leaf: MyTree) {}

    fn node(val: u8) -> MyTree {
        MyTree::Leaf(val, Rc::new(MyTree::Nil), Rc::new(MyTree::Nil))
    }
}

pub fn do_refcount() {
    let tree = MyTree::Leaf(
        5,
        Rc::new(MyTree::Nil),
        Rc::new(MyTree::Leaf(
            10,
            Rc::new(MyTree::Nil),
            Rc::new(MyTree::node(7)),
        )),
    );
    println!("{:?}", tree);
    println!("{:?}", tree.right().right());
    println!("{:?}", tree.right().left());
}

use std::rc::Rc;

enum MyTree {
    Leaf(u8, Rc<MyTree>, Rc<MyTree>),
    Nil,
}

pub fn do_refcount() {
    let tree = MyTree::Leaf(
        5,
        Rc::new(MyTree::Nil),
        Rc::new(MyTree::Leaf(10, Rc::new(MyTree::Nil), Rc::new(MyTree::Nil))),
    );
}

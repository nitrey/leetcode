use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

type NodeRefCell = Rc<RefCell<Node>>;

#[allow(dead_code)]
struct Node {
    pub key: i32,
    pub value: i32,
    pub prev: Option<NodeRefCell>,
    pub next: Option<NodeRefCell>,
}

#[allow(dead_code)]
impl Node {
    fn new(key: i32, value: i32) -> Node {
        Node { key, value, prev: None, next: None }
    }

    fn update_prev_next(self: &mut Self, prev: Option<NodeRefCell>, next: Option<NodeRefCell>) {
        self.prev = prev;
        self.next = next;
    }

    fn to_reference(self: Self) -> NodeRefCell {
        Rc::new(
            RefCell::new(
                self
            )
        )
    }

    fn connect(prev: &NodeRefCell, next: &NodeRefCell) {
        RefCell::borrow_mut(prev).next = Some(Rc::clone(next));
        RefCell::borrow_mut(next).prev = Some(Rc::clone(prev));
    }
}

#[allow(dead_code)]
struct LRUCache {
    capacity: usize,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    map: HashMap<i32, Rc<RefCell<Node>>>,
}

}
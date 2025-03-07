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
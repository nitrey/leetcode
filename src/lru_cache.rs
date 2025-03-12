use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NodeRefCell = Rc<RefCell<Node>>;

// https://leetcode.com/problems/lru-cache/
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
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }

    fn update_prev_next(self: &mut Self, prev: Option<NodeRefCell>, next: Option<NodeRefCell>) {
        self.prev = prev;
        self.next = next;
    }

    fn to_reference(self: Self) -> NodeRefCell {
        Rc::new(RefCell::new(self))
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

#[allow(dead_code)]
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            head: None,
            tail: None,
            map: HashMap::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let existing = self.map.get(&key).and_then(|x| Some(Rc::clone(x)));
        if let Some(node) = existing {
            let value = RefCell::borrow(&node).value;
            self.move_to_tail(&key, node);
            value
        } else {
            -1
        }
    }

    fn move_to_tail(&mut self, key: &i32, node: NodeRefCell) {
        let (tail, next) = {
            let tail = self.tail.clone();
            let next = RefCell::borrow(&node).next.clone();
            match (tail, next) {
                (Some(t), Some(n)) => (t.clone(), n.clone()),
                _ => return,
            }
        };
        let tail_key = RefCell::borrow(&tail).key;
        // let head_key: i32 = RefCell::borrow(&head).key;
        if tail_key == *key {
            // Already at the end
            return;
        }
        if let Some(prev) = RefCell::borrow(&node).prev.clone() {
            Node::connect(&prev, &next);
        } else {
            // In this case the Node was a head
            self.head = Some(next.clone());
            RefCell::borrow_mut(&next).prev = None;
        }
        Node::connect(&tail, &node);
        RefCell::borrow_mut(&node).next = None;
        self.tail = Some(node);
    }

    fn put(&mut self, key: i32, value: i32) {
        let existing = self.map.get(&key).and_then(|x| Some(Rc::clone(x)));
        match existing {
            None => {
                let new_node = Node::new(key, value).to_reference();
                if self.map.len() == self.capacity {
                    if self.capacity == 1 {
                        let head = self.head.clone().unwrap();
                        let key_to_remove = RefCell::borrow(&head).key;
                        self.map.remove(&key_to_remove);
                        self.head = None;
                        self.tail = None;
                    } else {
                        let head = self.head.clone().unwrap();
                        let key_to_remove = RefCell::borrow(&head).key;
                        self.map.remove(&key_to_remove);
                        let head_next = RefCell::borrow(&head).next.clone().unwrap();
                        RefCell::borrow_mut(&head_next).prev = None;
                        self.head = Some(head_next);
                    }
                }
                if let Some(tail) = self.tail.clone() {
                    RefCell::borrow_mut(&tail).next = Some(new_node.clone());
                    RefCell::borrow_mut(&new_node).prev = Some(tail.clone());
                    self.tail = Some(new_node.clone());
                } else {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node.clone());
                }
                self.map.insert(key, new_node);
            }
            Some(node) => {
                RefCell::borrow_mut(&node).value = value;
                self.move_to_tail(&key, node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let capacity = 2;
        let mut cache = LRUCache::new(capacity);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}

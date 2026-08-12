use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct BPTreeNode {
    is_leaf: bool,
    keys: Vec<u32>,
    children: Vec<Rc<RefCell<BPTreeNode>>>,
    next: Option<Rc<RefCell<BPTreeNode>>>,
}

pub struct BPTree {
    pub root: Option<BPTreeNode>,
}

const M: usize = 4;

impl BPTreeNode {
    fn new(is_leaf: bool) -> Self {
        Self {
            is_leaf,
            keys: Vec::with_capacity(M - 1),
            children: Vec::with_capacity(M),
            next: None,
        }
    }

    fn split_internal_child(&mut self, child_idx: usize) {
        let child = self.children.remove(child_idx);
        let split_point = (child.borrow().keys.len() + 1) / 2;
        let mid_key = child.borrow().keys[split_point - 1];

        let mut right_sibling = BPTreeNode::new(false);
        right_sibling.keys = child.borrow_mut().keys.split_off(split_point);
        right_sibling.children = child.borrow_mut().children.split_off(split_point);

        _ = child.borrow_mut().keys.pop();

        self.keys.insert(child_idx, mid_key);
        self.children.insert(child_idx, child);
        self.children
            .insert(child_idx + 1, Rc::new(RefCell::new(right_sibling)));
    }

    fn split_leaf_child(&mut self, child_idx: usize) {
        let child = self.children.remove(child_idx);
        let split_point = (child.borrow().keys.len() + 1) / 2;

        let mut right_sibling = BPTreeNode::new(true);
        right_sibling.keys = child.borrow_mut().keys.split_off(split_point);

        let mid_key = right_sibling.keys[0];

        self.keys.insert(child_idx, mid_key);
        self.children.insert(child_idx, child);
        self.children
            .insert(child_idx + 1, Rc::new(RefCell::new(right_sibling)));

        let right = Rc::clone(&self.children[child_idx + 1]);
        let left = &mut self.children[child_idx];

        right.borrow_mut().next = left.borrow_mut().next.take();
        left.borrow_mut().next = Some(right);
    }

    fn split_child(&mut self, child_idx: usize) {
        let child = &self.children[child_idx];

        if child.borrow().is_leaf {
            self.split_leaf_child(child_idx);
        } else {
            self.split_internal_child(child_idx);
        }
    }

    fn insert(&mut self, key: u32) -> bool {
        if self.is_leaf {
            if self.keys.contains(&key) {
                return false;
            }
            self.keys.push(key);
            self.keys.sort();
            return self.keys.len() == M;
        }

        let idx = match self.keys.binary_search(&key) {
            Ok(_) => return false,
            Err(i) => i,
        };

        let child_overflowed = self.children[idx].borrow_mut().insert(key);

        if child_overflowed {
            self.split_child(idx);

            return self.keys.len() == M;
        }

        false
    }
}

impl BPTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: u32) {
        match &mut self.root {
            None => {
                let mut node = BPTreeNode::new(true);
                node.keys.push(key);
                self.root = Some(node);
            }
            Some(root) => {
                let overflow = root.insert(key);

                if overflow {
                    let old_root = self.root.take().unwrap();
                    let mut new_root = BPTreeNode::new(false);
                    new_root.children.push(Rc::new(RefCell::new(old_root)));
                    new_root.split_child(0);
                    self.root = Some(new_root);
                }
            }
        }
    }
}

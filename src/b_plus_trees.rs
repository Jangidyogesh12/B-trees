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

    fn search(&self, key: u32) -> bool {
        if self.is_leaf {
            return self.keys.binary_search(&key).is_ok();
        }

        let idx = match self.keys.binary_search(&key) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        };

        self.children[idx].borrow().search(key)
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

    fn borrow_from_prev(&mut self, child_idx: usize) {
        let (left, right) = self.children.split_at_mut(child_idx);
        let sibling = &mut left[child_idx - 1];
        let child = &mut right[0];

        if !child.borrow().is_leaf {
            let last_child = sibling.borrow_mut().children.pop();
            child.borrow_mut().children.insert(0, last_child.unwrap());
            let parent_key = self.keys[child_idx - 1];
            self.keys[child_idx - 1] = sibling.borrow_mut().keys.pop().unwrap();
            child.borrow_mut().keys.insert(0, parent_key);
        } else {
            let parent_key = sibling.borrow_mut().keys.pop().unwrap();
            child.borrow_mut().keys.insert(0, parent_key);
            self.keys[child_idx - 1] = parent_key;
        }
    }

    fn borrow_from_next(&mut self, child_idx: usize) {
        let (left, right) = self.children.split_at_mut(child_idx + 1);
        let sibling = &mut right[0];
        let child = &mut left[child_idx];

        if !child.borrow().is_leaf {
            let first_child = sibling.borrow_mut().children.remove(0);
            child.borrow_mut().children.push(first_child);

            let parent_key = self.keys[child_idx];
            self.keys[child_idx] = sibling.borrow_mut().keys.remove(0);
            child.borrow_mut().keys.push(parent_key);
        } else {
            child
                .borrow_mut()
                .keys
                .push(sibling.borrow_mut().keys.remove(0));
            let parent_key = sibling.borrow().keys[0];
            self.keys[child_idx] = parent_key;
        }
    }

    fn merge(&mut self, child_idx: usize) {
        let left = self.children.remove(child_idx);
        let right = self.children.remove(child_idx);
        let parent_key = self.keys.remove(child_idx);

        if left.borrow().is_leaf {
            left.borrow_mut()
                .keys
                .extend(right.borrow().keys.iter().copied());
            left.borrow_mut().next = right.borrow().next.clone();
        } else {
            left.borrow_mut().keys.push(parent_key);
            left.borrow_mut()
                .keys
                .extend(right.borrow().keys.iter().copied());
            left.borrow_mut()
                .children
                .extend(right.borrow().children.clone());
        }

        self.children.insert(child_idx, left);
    }

    fn ensure_child_has_enough_keys(&mut self, child_idx: usize) -> usize {
        let min_keys = (M / 2) - 1;

        if self.children[child_idx].borrow().keys.len() > min_keys {
            return child_idx;
        }

        if child_idx > 0 && self.children[child_idx - 1].borrow().keys.len() > min_keys {
            self.borrow_from_prev(child_idx);
            return child_idx;
        }

        if child_idx < (self.children.len() - 1)
            && self.children[child_idx + 1].borrow().keys.len() > min_keys
        {
            self.borrow_from_next(child_idx);
            return child_idx;
        }

        if child_idx > 0 {
            self.merge(child_idx - 1);
            return child_idx - 1;
        } else {
            self.merge(child_idx);
            return child_idx;
        }
    }

    fn delete(&mut self, key: u32) -> bool {
        let min_keys = (M / 2) - 1;

        if self.is_leaf {
            // Case 1 key is in the leaf node
            if let Ok(idx) = self.keys.binary_search(&key) {
                self.keys.remove(idx);
                return self.keys.len() < min_keys;
            }

            // Key not found in the the leaf node
            return false;
        }

        // Key is in the internal node or we need to descend
        match self.keys.binary_search(&key) {
            Ok(idx) => {
                // If key is in the internal node
                let new_idx = self.ensure_child_has_enough_keys(idx + 1);
                return self.children[new_idx].borrow_mut().delete(key);
            }
            Err(idx) => {
                // If key is not in the internal node descend
                let new_idx = self.ensure_child_has_enough_keys(idx);

                return self.children[new_idx].borrow_mut().delete(key);
            }
        };
    }
}

impl BPTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn search(&self, key: u32) -> bool {
        match &self.root {
            None => false,
            Some(root) => root.search(key),
        }
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

    pub fn delete(&mut self, key: u32) {
        let root = match &mut self.root {
            None => return,
            Some(root) => root,
        };

        let _need_rebalancing = root.delete(key);

        if root.keys.is_empty() && !root.children.is_empty() {
            let old_root = self.root.take().unwrap();

            let mut children = old_root.children;

            if children.len() == 1 {
                self.root = Some(Rc::try_unwrap(children.remove(0)).unwrap().into_inner());
            }
        }
    }
}

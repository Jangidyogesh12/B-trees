#[derive(Debug)]
pub struct BTreeNode {
    is_leaf: bool,
    keys: Vec<u32>,
    children: Vec<Box<BTreeNode>>,
}

pub struct BTree {
    pub root: Option<BTreeNode>,
}

const M: usize = 4;

impl BTreeNode {
    fn new(is_leaf: bool) -> Self {
        Self {
            is_leaf,
            keys: Vec::with_capacity(M - 1),
            children: Vec::with_capacity(M),
        }
    }

    fn search(&self, key: u32) -> bool {
        match self.keys.binary_search(&key) {
            Ok(_) => true,
            Err(idx) => {
                if self.is_leaf {
                    return false;
                } else {
                    self.children[idx].search(key)
                }
            }
        }
    }

    fn split_child(&mut self, child_idx: usize) {
        let mut child = self.children.remove(child_idx);
        let split_point = (child.keys.len() + 1) / 2;
        let median_key = child.keys[split_point - 1];
        let mut right_sibling = BTreeNode::new(child.is_leaf);

        right_sibling.keys = child.keys.split_off(split_point);

        let _ = child.keys.pop();

        if !child.is_leaf {
            right_sibling.children = child.children.split_off(split_point);
        }

        self.keys.insert(child_idx, median_key);

        self.children.insert(child_idx, child);

        self.children.insert(child_idx + 1, Box::new(right_sibling));
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

        let child_overflowed = self.children[idx].insert(key);

        if child_overflowed {
            self.split_child(idx);
            return self.keys.len() == M;
        }

        false
    }

    fn get_predecessor(&self, idx: usize) -> u32 {
        let mut node = &self.children[idx];

        while !node.is_leaf {
            node = &node.children[node.children.len() - 1];
        }

        *node.keys.last().unwrap()
    }

    fn get_successor(&self, idx: usize) -> u32 {
        let mut node = &self.children[idx + 1];

        while !node.is_leaf {
            node = &node.children[0];
        }

        *node.keys.first().unwrap()
    }

    fn borrow_from_prev(&mut self, idx: usize) {
        let (left, right) = self.children.split_at_mut(idx);

        let sibling = &mut left[idx - 1];
        let child = &mut right[0];

        if !sibling.is_leaf {
            let last_child = sibling.children.pop().unwrap();
            child.children.insert(0, last_child);
        }

        let parent_key = self.keys[idx - 1];
        self.keys[idx - 1] = sibling.keys.pop().unwrap();
        child.keys.insert(0, parent_key);
    }

    fn borrow_from_next(&mut self, idx: usize) {
        let (left, right) = self.children.split_at_mut(idx + 1);

        let child = &mut left[idx];
        let sibling = &mut right[0];

        if !sibling.is_leaf {
            let first_key = sibling.children.remove(0);
            child.children.push(first_key);
        }

        let parent_key = self.keys[idx];
        self.keys[idx] = sibling.keys.remove(0);
        child.keys.push(parent_key);
    }

    fn merge(&mut self, idx: usize) {
        let mut left = self.children.remove(idx);
        let right = self.children.remove(idx);

        let key = self.keys.remove(idx);

        left.keys.push(key);

        left.keys.extend(right.keys);

        left.children.extend(right.children);

        self.children.insert(idx, left);
    }

    fn ensure_child_has_enough_keys(&mut self, idx: usize) -> usize {
        // in here the idx is child_idx not the key_idx
        let min_keys = (M / 2) - 1;

        // if child allredy have enogh keys return do nothing
        if self.children[idx].keys.len() > min_keys {
            return idx;
        }

        // try to borrow from the left
        if idx > 0 && self.children[idx - 1].keys.len() > min_keys {
            self.borrow_from_prev(idx);
            return idx;
        }

        // try to borrow from the right
        if idx < self.children.len() - 1 && self.children[idx + 1].keys.len() > min_keys {
            self.borrow_from_next(idx);
            return idx;
        }

        // if cannot borrow - must merge with the siblings
        if idx > 0 {
            self.merge(idx - 1);
            return idx - 1;
        } else {
            self.merge(idx);
            return idx;
        }
    }

    fn delete(&mut self, key: u32) -> bool {
        // minimum number of keys
        let min_keys = (M / 2) - 1;

        if self.is_leaf {
            // Case 1 : key is in leaf node
            if let Ok(idx) = self.keys.binary_search(&key) {
                self.keys.remove(idx);
                return self.keys.len() < min_keys;
            }

            // key not found in leaf
            return false;
        }

        // Case 2 and 3 : key is in the internal node or we need to descend
        let idx = match self.keys.binary_search(&key) {
            Ok(idx) => idx,  // Key found in this internal node
            Err(idx) => idx, // Key should be in child[idx]
        };

        // Case 3: Key is not in this node - descend to child
        if idx == self.keys.len() || self.keys[idx] != key {
            let new_idx = self.ensure_child_has_enough_keys(idx);

            return self.children[new_idx].delete(key);
        }

        // Case 2 : Key is in the internal node

        // replace with its predecessor
        if self.children[idx].keys.len() > min_keys {
            let predecessor = self.get_predecessor(idx);
            self.keys[idx] = predecessor;
            return self.children[idx].delete(predecessor);
        }

        // replace with its successor
        if self.children[idx + 1].keys.len() > min_keys {
            let successor = self.get_successor(idx);
            self.keys[idx] = successor;
            return self.children[idx + 1].delete(successor);
        }

        // if both have minimum number of keys then merge
        self.merge(idx);

        let merge_id = idx;

        self.children[merge_id].delete(key)
    }
}

impl BTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn search(&self, key: u32) -> bool {
        match &self.root {
            None => false,
            Some(node) => node.search(key),
        }
    }

    pub fn insert(&mut self, key: u32) {
        match &mut self.root {
            None => {
                let mut node = BTreeNode::new(true);
                node.keys.push(key);
                self.root = Some(node);
            }
            Some(root) => {
                let overflow = root.insert(key);

                if overflow {
                    let old_root = self.root.take().unwrap();

                    let mut new_root = BTreeNode::new(false);

                    new_root.children.push(Box::new(old_root));

                    new_root.split_child(0);

                    self.root = Some(new_root);
                }
            }
        }
    }

    pub fn delete(&mut self, key: u32) {
        // Check if tree is empty
        let root = match &mut self.root {
            None => return,
            Some(root) => root,
        };

        // delete the key recursively
        let _need_rebalancing = root.delete(key);

        // if the rood has no key left but has children
        // the tree height should decrease

        if root.keys.is_empty() && !root.children.is_empty() {
            let old_root = self.root.take().unwrap();

            let mut children = old_root.children;

            if children.len() == 1 {
                self.root = Some(*children.remove(0));
            }
        }
    }
}

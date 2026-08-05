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
}

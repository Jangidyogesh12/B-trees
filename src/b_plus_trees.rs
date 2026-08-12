#[derive(Debug)]
pub struct BPTreeNode {
    is_leaf: bool,
    keys: Vec<u32>,
    children: Vec<Box<BPTreeNode>>,
    next: Option<Box<BPTreeNode>>,
}

pub struct BPTree {
    root: Option<BPTreeNode>,
    head: Option<Box<BPTreeNode>>,
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
        let mut child = self.children.remove(child_idx);
        let split_point = (child.keys.len() + 1) / 2;
        let mid_key = child.keys[split_point - 1];

        let mut right_sibling = BPTreeNode::new(false);
        right_sibling.keys = child.keys.split_off(split_point);
        right_sibling.children = child.children.split_off(split_point);

        _ = child.keys.pop();

        self.keys.insert(child_idx, mid_key);
        self.children.insert(child_idx, child);
        self.children.insert(child_idx + 1, Box::new(right_sibling));
    }

    fn split_leaf_child(&mut self, child_idx: usize) {
        let mut child = self.children.remove(child_idx);
        let split_point = (child.keys.len() + 1) / 2;

        let mut right_sibling = BPTreeNode::new(true);
        right_sibling.keys = child.keys.split_off(split_point);

        let mid_key = right_sibling.keys[0];

        self.keys.insert(child_idx, mid_key);
        self.children.insert(child_idx, child);
        self.children.insert(child_idx + 1, Box::new(right_sibling));

        let left = &mut self.children[child_idx];
        let mut right = self.children.remove(child_idx + 1);

        right.next = left.next.take();
        left.next = Some(right);
    }
}

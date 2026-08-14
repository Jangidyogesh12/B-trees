use crate::b_plus_trees::BPTree;

pub mod b_plus_trees;
pub mod b_tree;
pub mod buffer_pool;
pub mod clock;
pub mod lru_cache;

fn main() {
    let mut tree = BPTree::new();

    // Insert keys to create a multi-level B-tree
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4); // This causes a root split!
    tree.insert(5);
    tree.insert(6);
    tree.insert(7);
    tree.insert(8);
    tree.insert(9);
    tree.insert(10);
    tree.insert(11);
    tree.insert(12);
    tree.insert(13);
    tree.insert(14);
    tree.insert(15);
    tree.insert(16);

    println!("{:?}", tree.root);

    let a = tree.search(18);

    println!("{a}")
}

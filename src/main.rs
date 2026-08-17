use crate::b_plus_trees::BPTree;

pub mod b_plus_trees;
pub mod b_tree;

fn main() {
    let mut tree = BPTree::new();

    // Insert keys to create a multi-level B-tree
    tree.insert(20);
    tree.insert(40);
    tree.insert(50);
    tree.insert(80); // This causes a root split!
    tree.insert(90);
    tree.insert(100);
    tree.delete(100);
    tree.delete(50);

    println!("{:?}", tree.root);

    let v = vec![2, 3, 4, 5];

    let b: Vec<i32> = v.into_iter().map(|x| x * 2).collect();

    println!("{:?}", b);
}

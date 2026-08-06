use crate::b_tree::BTree;

pub mod b_tree;
pub mod buffer_pool;
pub mod clock;
pub mod lru_cache;

fn main() {
    let v = vec![1, 3];

    let (a, b) = v.split_at(1);

    println!("a = {:?}", a);

    println!("b = {:?}", b);
    let mut tree = BTree::new();

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

    // // Verify values can be successfully located
    // println!("=== Initial Tree ===");
    // println!("Contains 20? {}", tree.search(20)); // Output: false
    // println!("Contains 99? {}", tree.search(99)); // Output: false
    // println!("Contains 8? {}", tree.search(8)); // Output: true

    // // Demonstrate deletion
    // println!("\n=== Deleting key 8 ===");
    // tree.delete(8);
    // println!("Contains 8 after deletion? {}", tree.search(8)); // Output: false
    // println!("Contains 9? {}", tree.search(9)); // Output: true

    // println!("\n=== Deleting key 4 (internal node) ===");
    // tree.delete(4);
    // println!("Contains 4 after deletion? {}", tree.search(4)); // Output: false
    // println!("Contains 3? {}", tree.search(3)); // Output: true
    // println!("Contains 5? {}", tree.search(5)); // Output: true
}

/// B-Tree Implementation in Rust
///
/// A B-tree is a self-balancing tree data structure that maintains sorted data
/// and allows searches, sequential access, insertions, and deletions in
/// logarithmic time.
///
/// Properties:
/// - All leaves are at the same level
/// - A node with n keys has n+1 children
/// - Every node (except root) has at least ⌈M/2⌉ children
/// - Every node contains at most M-1 keys
/// - M is the order of the B-tree (maximum number of children)

/// Represents a single node in the B-tree
#[derive(Debug)]
pub struct BTreeNode {
    /// Whether this node is a leaf (no children)
    is_leaf: bool,
    /// The keys stored in this node (sorted in ascending order)
    keys: Vec<u32>,
    /// Child pointers - only used for internal nodes (is_leaf = false)
    /// children[i] contains all keys < keys[i]
    /// children[n] contains all keys > keys[n-1]
    children: Vec<Box<BTreeNode>>,
}

/// The B-tree structure containing the root node
pub struct BTree {
    pub root: Option<BTreeNode>,
}

/// Order of the B-tree (maximum number of children per node)
/// With M=4, each node has:
/// - At most 3 keys (M-1)
/// - At least 1 key (⌈M/2⌉-1) for non-root nodes
/// - At most 4 children (M)
/// - At least 2 children (⌈M/2⌉) for non-root internal nodes
const M: usize = 4;

impl BTreeNode {
    /// Creates a new empty B-tree node
    ///
    /// # Arguments
    /// * `is_leaf` - Whether this node is a leaf node
    ///
    /// # Returns
    /// A new empty BTreeNode with pre-allocated capacity
    fn new(is_leaf: bool) -> Self {
        Self {
            is_leaf,
            keys: Vec::with_capacity(M - 1),
            children: Vec::with_capacity(M),
        }
    }

    /// Searches for a key in the B-tree node
    ///
    /// Uses binary search for efficient key lookup.
    /// For leaf nodes: returns true if key is found
    /// For internal nodes: recursively searches the appropriate child
    ///
    /// # Arguments
    /// * `key` - The key to search for
    ///
    /// # Returns
    /// true if the key exists in the tree, false otherwise
    fn search(&self, key: u32) -> bool {
        // Binary search to find if key exists or where it should be
        match self.keys.binary_search(&key) {
            Ok(_) => true, // Key found in this node
            Err(idx) => {
                if self.is_leaf {
                    return false; // Leaf node and key not found
                } else {
                    // Internal node - search the appropriate child
                    // idx is where the key would be inserted, so children[idx]
                    // contains all keys less than keys[idx]
                    self.children[idx].search(key)
                }
            }
        }
    }

    /// Splits a child that has exceeded its maximum capacity
    ///
    /// When a child node has M keys (too many), we split it:
    /// 1. Find the median key
    /// 2. Move median up to parent
    /// 3. Split remaining keys into left and right halves
    ///
    /// # Arguments
    /// * `child_idx` - Index of the child to split
    fn split_child(&mut self, child_idx: usize) {
        // Take the overflowing child out of the parent
        let mut child = self.children.remove(child_idx);

        // Calculate split point: ceil((M-1+1)/2) = ceil(M/2)
        // For M=4: split_point = 2, so we take keys[0..2] and keys[3..]
        // The middle key (keys[2]) will be moved up to parent
        let split_point = (child.keys.len() + 1) / 2;

        // The median key that will be promoted to the parent
        let median_key = child.keys[split_point - 1];

        // Create right sibling to hold the upper half of keys
        let mut right_sibling = BTreeNode::new(child.is_leaf);

        // Move upper half of keys to right sibling
        // split_off returns the upper half, leaving lower half in child
        right_sibling.keys = child.keys.split_off(split_point);

        // Remove the median key from child (it's going to parent)
        let _ = child.keys.pop();

        // If this is an internal node, also split the children
        if !child.is_leaf {
            right_sibling.children = child.children.split_off(split_point);
        }

        // Insert the median key into parent at position child_idx
        self.keys.insert(child_idx, median_key);

        // Re-insert the (now smaller) child at child_idx
        self.children.insert(child_idx, child);

        // Insert the new right sibling after the child
        self.children
            .insert(child_idx + 1, Box::new(right_sibling));
    }

    /// Inserts a key into the B-tree node
    ///
    /// # Arguments
    /// * `key` - The key to insert
    ///
    /// # Returns
    /// true if this node overflowed (needs splitting), false otherwise
    fn insert(&mut self, key: u32) -> bool {
        // Case 1: Leaf node - insert directly
        if self.is_leaf {
            // Don't insert duplicates
            if self.keys.contains(&key) {
                return false;
            }
            self.keys.push(key);
            self.keys.sort();
            // Return true if node now has M keys (needs splitting)
            return self.keys.len() == M;
        }

        // Case 2: Internal node - find correct child to descend
        let idx = match self.keys.binary_search(&key) {
            Ok(_) => return false, // Key already exists, don't insert
            Err(i) => i, // Index where key should be inserted
        };

        // Recursively insert into the appropriate child
        let child_overflowed = self.children[idx].insert(key);

        // If child overflowed, split it
        if child_overflowed {
            self.split_child(idx);
            return self.keys.len() == M; // Check if this node now overflows
        }

        false
    }

    /// Finds the predecessor key (largest key in the left subtree)
    ///
    /// Used when deleting from internal nodes to find a replacement.
    /// The predecessor is the rightmost key in the left subtree.
    ///
    /// # Arguments
    /// * `idx` - Index of the key whose predecessor we want
    ///
    /// # Returns
    /// The predecessor key value
    fn get_predecessor(&self, idx: usize) -> u32 {
        // Start from the left child of the key at index idx
        let mut node = &self.children[idx];

        // Keep going to the rightmost leaf
        while !node.is_leaf {
            node = &node.children[node.children.len() - 1];
        }

        // The last key in the rightmost leaf is the predecessor
        *node.keys.last().unwrap()
    }

    /// Finds the successor key (smallest key in the right subtree)
    ///
    /// Used when deleting from internal nodes to find a replacement.
    /// The successor is the leftmost key in the right subtree.
    ///
    /// # Arguments
    /// * `idx` - Index of the key whose successor we want
    ///
    /// # Returns
    /// The successor key value
    fn get_successor(&self, idx: usize) -> u32 {
        // Start from the right child of the key at index idx
        let mut node = &self.children[idx + 1];

        // Keep going to the leftmost leaf
        while !node.is_leaf {
            node = &node.children[0];
        }

        // The first key in the leftmost leaf is the successor
        *node.keys.first().unwrap()
    }

    /// Borrows a key from the left sibling (previous sibling)
    ///
    /// This is called when a child has too few keys. We "borrow" the
    /// last key from the left sibling and move a parent key down.
    ///
    /// Visual representation:
    /// Before:
    /// ```
    ///     [parent_key]
    ///    /            \
    /// [sibling]     [child] (needs keys)
    ///  [a,b]         [d]  <- too few keys
    /// ```
    ///
    /// After:
    /// ```
    ///        [b]
    ///       /    \
    ///     [a]   [parent_key, d]
    /// ```
    ///
    /// # Arguments
    /// * `idx` - Index of the child that needs a key
    fn borrow_from_prev(&mut self, idx: usize) {
        // Get mutable references to the child and its left sibling
        // We use split_at_mut to get non-overlapping mutable references
        let (left, right) = self.children.split_at_mut(idx);
        let child = &mut right[0];
        let sibling = &mut left[idx - 1];

        // If sibling is not a leaf, also move its last child
        if !sibling.is_leaf {
            let last_child = sibling.children.pop().unwrap();
            child.children.insert(0, last_child);
        }

        // Move parent key down to child
        let parent_key = self.keys[idx - 1];

        // Move sibling's last key up to parent position
        self.keys[idx - 1] = sibling.keys.pop().unwrap();

        // Insert the old parent key at the beginning of child
        child.keys.insert(0, parent_key);
    }

    /// Borrows a key from the right sibling (next sibling)
    ///
    /// This is called when a child has too few keys. We "borrow" the
    /// first key from the right sibling and move a parent key down.
    ///
    /// Visual representation:
    /// Before:
    /// ```
    ///     [parent_key]
    ///    /            \
    ///  [child]     [sibling] (needs keys)
    ///  [d]        [f,g]  <- too few keys
    /// ```
    ///
    /// After:
    /// ```
    ///        [f]
    ///       /    \
    ///     [d, parent_key]   [g]
    /// ```
    ///
    /// # Arguments
    /// * `idx` - Index of the child that needs a key
    fn borrow_from_next(&mut self, idx: usize) {
        // Get mutable references to the child and its right sibling
        // We need to access children[idx] (child) and children[idx+1] (sibling)
        let (left, right) = self.children.split_at_mut(idx + 1);
        let child = &mut left[idx];
        let sibling = &mut right[0];

        // Move parent key down to child
        let parent_key = self.keys[idx];

        // Append parent key to child
        child.keys.push(parent_key);

        // Move sibling's first key up to parent position
        self.keys[idx] = sibling.keys.remove(0);

        // If sibling is not a leaf, also move its first child
        if !sibling.is_leaf {
            let first_child = sibling.children.remove(0);
            child.children.push(first_child);
        }
    }

    /// Merges two child nodes into one
    ///
    /// This is called when both siblings have minimum keys.
    /// We merge them by moving a parent key down and combining all keys.
    ///
    /// Visual representation:
    /// Before:
    /// ```
    ///        [parent_key]
    ///       /            \
    ///     [child]     [sibling]
    ///     [a,b]        [c,d]
    /// ```
    ///
    /// After:
    /// ```
    ///          [merged]
    ///         [a,b,parent_key,c,d]
    /// ```
    ///
    /// # Arguments
    /// * `idx` - Index of the left child (right child is at idx+1)
    fn merge(&mut self, idx: usize) {
        // Remove both children from parent
        let mut left = self.children.remove(idx);
        let right = self.children.remove(idx);

        // Remove the parent key that separates them
        let key = self.keys.remove(idx);

        // Add the parent key to the left child
        left.keys.push(key);

        // Move all keys from right child to left child
        left.keys.extend(right.keys);

        // Move all children from right child to left child (if internal)
        left.children.extend(right.children);

        // Re-insert the merged child at position idx
        self.children.insert(idx, left);
    }

    /// Ensures a child has at least t keys before we descend into it
    ///
    /// If the child has fewer than t keys, we either:
    /// 1. Borrow from a sibling (if sibling has more than t-1 keys)
    /// 2. Merge with a sibling (if both have t-1 keys)
    ///
    /// # Arguments
    /// * `idx` - Index of the child to ensure has enough keys
    fn ensure_child_has_enough_keys(&mut self, idx: usize) {
        // Minimum number of keys for a non-root node
        // For M=4, minimum = ⌈M/2⌉-1 = 1
        let min_keys = (M / 2) - 1;

        // If child already has enough keys, nothing to do
        if self.children[idx].keys.len() > min_keys {
            return;
        }

        // Try to borrow from left sibling
        if idx > 0 && self.children[idx - 1].keys.len() > min_keys {
            self.borrow_from_prev(idx);
            return;
        }

        // Try to borrow from right sibling
        if idx < self.children.len() - 1 && self.children[idx + 1].keys.len() > min_keys
        {
            self.borrow_from_next(idx);
            return;
        }

        // Cannot borrow - must merge with a sibling
        if idx > 0 {
            // Merge with left sibling
            self.merge(idx - 1);
        } else {
            // Merge with right sibling
            self.merge(idx);
        }
    }

    /// Deletes a key from the B-tree node
    ///
    /// This is the main deletion algorithm which handles three cases:
    ///
    /// Case 1: Key is in a leaf node
    ///   - If node has more than minimum keys, simply remove
    ///   - Otherwise, borrow from sibling or merge
    ///
    /// Case 2: Key is in an internal node
    ///   - If left child has more than minimum keys, replace with predecessor
    ///   - Else if right child has more than minimum keys, replace with successor
    ///   - Else merge both children and delete from merged node
    ///
    /// Case 3: Key is not in this node (internal node)
    ///   - Ensure the child we descend into has enough keys
    ///   - Recursively delete from that child
    ///
    /// # Arguments
    /// * `key` - The key to delete
    ///
    /// # Returns
    /// true if this node now has fewer keys than allowed (needs rebalancing)
    fn delete(&mut self, key: u32) -> bool {
        // Minimum keys for a non-root node
        let min_keys = (M / 2) - 1;

        if self.is_leaf {
            // Case 1: Key is in a leaf node
            if let Ok(idx) = self.keys.binary_search(&key) {
                self.keys.remove(idx);
                // Return true if node now has too few keys
                return self.keys.len() < min_keys;
            }
            // Key not found in leaf
            return false;
        }

        // Case 2 & 3: Key is in an internal node or we need to descend
        let idx = match self.keys.binary_search(&key) {
            Ok(idx) => idx, // Key found in this internal node
            Err(idx) => idx, // Key should be in child[idx]
        };

        // Case 3: Key is not in this node - descend to child
        if idx == self.keys.len() || self.keys[idx] != key {
            // Ensure child has enough keys before descending
            self.ensure_child_has_enough_keys(idx);

            // After merge, the tree structure may have changed
            // Check if the child we want to descend into still exists
            let children_len = self.children.len();
            if idx >= children_len {
                // The merge changed the structure, try the last child
                return self.children[children_len - 1].delete(key);
            }

            // Recursively delete from the child
            return self.children[idx].delete(key);
        }

        // Case 2: Key is in this internal node
        // Now we need to find a replacement (predecessor or successor)

        // Subcase 2a: If left child has more than minimum keys,
        // replace key with its predecessor
        if self.children[idx].keys.len() > min_keys {
            let predecessor = self.get_predecessor(idx);
            self.keys[idx] = predecessor;
            // Recursively delete the predecessor from the left subtree
            return self.children[idx].delete(predecessor);
        }

        // Subcase 2b: If right child has more than minimum keys,
        // replace key with its successor
        if self.children[idx + 1].keys.len() > min_keys {
            let successor = self.get_successor(idx);
            self.keys[idx] = successor;
            // Recursively delete the successor from the right subtree
            return self.children[idx + 1].delete(successor);
        }

        // Subcase 2c: Both children have minimum keys
        // Merge the key and right child into the left child
        self.merge(idx);

        // Now delete the key from the merged left child
        // (which now contains the key we wanted to delete)
        // After merge, children[idx] is the merged node
        let merged_idx = idx;
        self.children[merged_idx].delete(key)
    }
}

impl BTree {
    /// Creates a new empty B-tree
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Searches for a key in the B-tree
    ///
    /// # Arguments
    /// * `key` - The key to search for
    ///
    /// # Returns
    /// true if the key exists in the tree, false otherwise
    pub fn search(&self, key: u32) -> bool {
        match &self.root {
            None => false,
            Some(node) => node.search(key),
        }
    }

    /// Inserts a key into the B-tree
    ///
    /// Handles the case where the root needs to be split by creating
    /// a new root level.
    ///
    /// # Arguments
    /// * `key` - The key to insert
    pub fn insert(&mut self, key: u32) {
        match &mut self.root {
            None => {
                // Tree is empty - create a new root leaf node
                let mut node = BTreeNode::new(true);
                node.keys.push(key);
                self.root = Some(node);
            }
            Some(root) => {
                // Insert into existing tree
                let overflow = root.insert(key);

                // If root overflowed, we need to create a new root
                if overflow {
                    // Take the old root out
                    let old_root = self.root.take().unwrap();

                    // Create new root with no keys yet
                    let mut new_root = BTreeNode::new(false);

                    // Add old root as first child
                    new_root.children.push(Box::new(old_root));

                    // Split the first child (this will move median up)
                    new_root.split_child(0);

                    // Set new root
                    self.root = Some(new_root);
                }
            }
        }
    }

    /// Deletes a key from the B-tree
    ///
    /// This method handles:
    /// 1. Finding the key to delete
    /// 2. Handling the case where root becomes empty after deletion
    /// 3. Calling the recursive delete algorithm
    ///
    /// # Arguments
    /// * `key` - The key to delete
    pub fn delete(&mut self, key: u32) {
        // Check if tree is empty
        let root = match &mut self.root {
            None => return,
            Some(root) => root,
        };

        // Delete the key recursively
        let _needs_rebalancing = root.delete(key);

        // If root has no keys left but has children,
        // the tree height should decrease
        if root.keys.is_empty() && !root.children.is_empty() {
            // Root is empty but has one child - make that child the new root
            let old_root = self.root.take().unwrap();
            let mut children = old_root.children;
            if children.len() == 1 {
                self.root = Some(*children.remove(0));
            }
        }
    }

    /// Prints the B-tree structure (for debugging)
    pub fn print_tree(&self) {
        match &self.root {
            None => println!("Tree is empty"),
            Some(root) => {
                println!("B-Tree (order {}):", M);
                Self::print_node(root, 0);
            }
        }
    }

    /// Recursively prints a node and its children
    fn print_node(node: &BTreeNode, level: usize) {
        let indent = "  ".repeat(level);
        let node_type = if node.is_leaf { "Leaf" } else { "Internal" };

        println!("{}{}: {:?}", indent, node_type, node.keys);

        if !node.is_leaf {
            for (i, child) in node.children.iter().enumerate() {
                print!("{}Child {}:", indent, i);
                Self::print_node(child, level + 1);
            }
        }
    }
}


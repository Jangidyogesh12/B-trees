# B-Tree Deletion: A Comprehensive Guide

## Table of Contents
1. [B-Tree Properties](#b-tree-properties)
2. [Deletion Overview](#deletion-overview)
3. [Case 1: Key in Leaf Node](#case-1-key-in-leaf-node)
4. [Case 2: Key in Internal Node](#case-2-key-in-internal-node)
5. [Case 3: Key Not Found](#case-3-key-not-found)
6. [Visual Examples](#visual-examples)
7. [Algorithm Pseudocode](#algorithm-pseudocode)
8. [Code Implementation](#code-implementation)

---

## B-Tree Properties

A B-tree of order **M** has the following properties:

| Property | Description |
|----------|-------------|
| **Maximum keys per node** | M - 1 |
| **Minimum keys per node** | ⌈M/2⌉ - 1 (except root) |
| **Maximum children per node** | M |
| **Minimum children per node** | ⌈M/2⌉ (except root) |
| **All leaves** | At the same level |

### For Our Implementation (M = 4)

- **Maximum keys:** 3
- **Minimum keys:** 1 (non-root)
- **Maximum children:** 4
- **Minimum children:** 2 (non-root internal)

---

## Deletion Overview

Deletion in a B-tree follows these principles:

1. **Find the key** to delete
2. **If key is in a leaf:** Remove it directly (with possible rebalancing)
3. **If key is in an internal node:** Replace with predecessor/succor, then delete that
4. **Rebalancing:** Borrow from siblings or merge nodes to maintain B-tree properties

### Key Insight

> When deleting, we always ensure we don't violate the minimum key requirement.
> If a node would have too few keys, we fix it BEFORE descending.

---

## Case 1: Key in Leaf Node

### Scenario
The key to delete is in a leaf node.

### Subcase 1a: Node has more than minimum keys
Simply remove the key. No rebalancing needed.

```
Before:  [5, 8, 12]     (3 keys, minimum is 1)
Delete:  8
After:   [5, 12]        (2 keys, still ≥ 1) ✓
```

### Subcase 1b: Node has exactly minimum keys
We need to borrow or merge before deleting.

#### Option 1: Borrow from Left Sibling
If the left sibling has more than minimum keys:

```
Before:
        [10]
       /    \
   [3,6]    [12,15]  <- wants to delete 12

Action: Borrow from left sibling
After:
        [6]
       /    \
     [3]    [10,12,15]  <- now delete 12
Delete 12:
        [6]
       /    \
     [3]    [10,15]  ✓
```

#### Option 2: Borrow from Right Sibling
If the right sibling has more than minimum keys:

```
Before:
        [10]
       /    \
   [3,6]    [12,15]  <- wants to delete 3

Action: Borrow from right sibling
After:
        [12]
       /    \
   [3,6,10]  [15]  <- now delete 3
Delete 3:
        [12]
       /    \
   [6,10]   [15]  ✓
```

#### Option 3: Merge with Sibling
If neither sibling can lend a key, merge with a sibling:

```
Before:
        [10]
       /    \
   [3]      [12,15]  <- wants to delete 3 (left has min keys)

Action: Merge with right sibling
After:
        [10]
       /    \
   [3,12,15]        <- merged node
Delete 3:
        [10]
       /    \
   [12,15]          ✓
```

---

## Case 2: Key in Internal Node

When the key to delete is in an internal node, we can't just remove it
because that would leave a gap. Instead, we find a replacement.

### Subcase 2a: Left Child has more than minimum keys
Replace with **predecessor** (rightmost key in left subtree).

```
Before:
        [10]
       /    \
   [3,6]    [12,15]
   
Delete: 10

Step 1: Find predecessor (rightmost in left subtree = 6)
Step 2: Replace 10 with 6
        [6]
       /    \
   [3]      [12,15]

Step 3: Delete 6 from where it was (left subtree)
        [6]
       /    \
   [3]      [12,15]  ✓
```

### Subcase 2b: Right Child has more than minimum keys
Replace with **successor** (leftmost key in right subtree).

```
Before:
        [10]
       /    \
   [3,6]    [12,15]
   
Delete: 10

Step 1: Find successor (leftmost in right subtree = 12)
Step 2: Replace 10 with 12
        [12]
       /    \
   [3,6]    [15]

Step 3: Delete 12 from where it was (right subtree)
        [12]
       /    \
   [3,6]    [15]  ✓
```

### Subcase 2c: Both Children have minimum keys
Merge the children, then delete from the merged node.

```
Before:
        [10]
       /    \
   [3]      [15]    <- both have minimum keys
   
Delete: 10

Step 1: Merge children with parent key
        (empty root)
       /         \
   [3, 10, 15]          <- merged node

Step 2: Delete 10 from merged node
        (empty root)
       /         \
   [3, 15]          ✓
```

---

## Case 3: Key Not Found

When the key doesn't exist in the current node but we know which child
to search:

1. **Ensure child has enough keys** before descending (use Case 1 logic)
2. **Recursively search and delete** in the child

```
Before:
        [10, 20]
       /   |   \
   [3,6] [15] [25,30]
   
Delete: 15

Step 1: Find which child should have 15 (child[1])
Step 2: Child[1] has minimum keys, so ensure it has enough
        - Try to borrow from sibling
        - Or merge with sibling
Step 3: Recursively delete 15 from child[1]
```

---

## Visual Examples

### Example 1: Simple Leaf Deletion

```
Initial Tree (M=4, max 3 keys per node):
        [50]
       /    \
   [10,30]  [70,90]

Delete 30:
        [50]
       /    \
   [10]     [70,90]  ✓
```

### Example 2: Deletion Requiring Borrow

```
Initial Tree:
          [50]
         /    \
    [10,30]   [70,90]
   /    \
 [5]   [20,25]

Delete 5:
        [50]
       /    \
  [10,30]   [70,90]
   /    \
 [20,25]  (no change needed, 5 was in leaf with 1 key)
 
Actually, need to rebalance first:
        [50]
       /    \
  [10,30]   [70,90]
   /    \
 [5,20,25]  <- borrow from sibling or merge

After delete:
        [50]
       /    \
  [10,30]   [70,90]
   /    \
 [20,25]  ✓
```

### Example 3: Deletion Requiring Merge

```
Initial Tree:
          [50]
         /    \
    [10,30]   [70,90]
   /    \     /    \
 [5]   [20] [60]  [80]

Delete 5:
        [50]
       /    \
  [10,30]   [70,90]
   /    \     /    \
 [20]  (merge needed)
 
After rebalancing:
          [50]
         /    \
    [10,30]   [70,90]
     /  \      /    \
   [20]  []  [60]  [80]
   
After merge:
          [50]
         /    \
    [10,30]   [70,90]
     /    \     /    \
   [20]  [60] [80]   ✓
```

### Example 4: Internal Node Deletion

```
Initial Tree:
            [50]
           /    \
      [20,30]   [70,80]
     /   |   \
  [10] [25] [40]

Delete 30 (internal node):
1. Left child [20] has only 1 key (min)
2. Right child [40] has only 1 key (min)
3. Merge children: [20, 30, 40]
4. Delete 30 from merged node

After:
            [50]
           /    \
      [20,40]   [70,80]
     /   |   \
  [10] [25]  (merged)  ✓
```

---

## Algorithm Pseudocode

```
DELETE(node, key):
    if node is leaf:
        // Case 1: Key in leaf
        if key in node.keys:
            remove key from node.keys
            if |node.keys| < t-1:  // t = ⌈M/2⌉
                return NEEDS_REBALANCING
            return OK
        else:
            return OK  // Key not found in leaf
    
    // Case 2 & 3: Key in internal node
    find index i where keys[i] >= key
    
    if keys[i] == key:
        // Case 2: Key is in this internal node
        if children[i] has > t-1 keys:
            // Subcase 2a: Replace with predecessor
            pred = GET_PREDECESSOR(children[i])
            keys[i] = pred
            DELETE(children[i], pred)
        
        else if children[i+1] has > t-1 keys:
            // Subcase 2b: Replace with successor
            succ = GET_SUCCESSOR(children[i+1])
            keys[i] = succ
            DELETE(children[i+1], succ)
        
        else:
            // Subcase 2c: Both children have min keys
            MERGE(children[i], keys[i], children[i+1])
            remove keys[i] from node
            DELETE(children[i], key)
    
    else:
        // Case 3: Key is not in this node
        ENSURE_CHILD_HAS_ENOUGH_KEYS(children[i])
        DELETE(children[i], key)

ENSURE_CHILD_HAS_ENOUGH_KEYS(child):
    if |child.keys| >= t:
        return  // Already has enough
    
    // Try to borrow from left sibling
    if left_sibling exists and |left_sibling.keys| > t-1:
        BORROW_FROM_LEFT(child)
        return
    
    // Try to borrow from right sibling
    if right_sibling exists and |right_sibling.keys| > t-1:
        BORROW_FROM_RIGHT(child)
        return
    
    // Cannot borrow - merge with a sibling
    if left_sibling exists:
        MERGE(left_sibling, child)
    else:
        MERGE(child, right_sibling)

BORROW_FROM_LEFT(child):
    // Move parent key down to child
    // Move sibling's last key up to parent
    // If sibling is internal, also move its last child

BORROW_FROM_RIGHT(child):
    // Move parent key down to child
    // Move sibling's first key up to parent
    // If sibling is internal, also move its first child

MERGE(left, right):
    // Take parent key that separates them
    // Add all keys from right to left
    // Add all children from right to left (if internal)
    // Remove right node from parent
```

---

## Code Implementation

### Key Functions in `b_tree.rs`

| Function | Purpose |
|----------|---------|
| `delete()` | Main deletion logic, handles all 3 cases |
| `ensure_child_has_enough_keys()` | Prepares child for descent by borrowing/merging |
| `borrow_from_prev()` | Borrows key from left sibling |
| `borrow_from_next()` | Borrows key from right sibling |
| `merge()` | Merges two children into one |
| `get_predecessor()` | Finds rightmost key in left subtree |
| `get_successor()` | Finds leftmost key in right subtree |

### Time Complexity

| Operation | Time Complexity |
|-----------|----------------|
| Search | O(log n) |
| Insert | O(log n) |
| Delete | O(log n) |
| Borrow/Merge | O(1) |

The delete operation is O(log n) because:
1. We descend from root to leaf: O(log n) levels
2. At each level, we do O(1) work (borrow or merge)
3. Total: O(log n)

---

## Common Pitfalls

1. **Forgetting to handle root becoming empty**
   - After deletion, if root has no keys but has children, reduce tree height

2. **Not rebalancing before descending**
   - Always ensure child has enough keys BEFORE descending into it

3. **Off-by-one errors in merge**
   - When merging, remember to remove the separating key from parent

4. **Not handling duplicates**
   - Our implementation prevents duplicate insertions

5. **Forgetting to move children during borrow**
   - When borrowing, also move the appropriate child pointer

---

## Testing

The implementation includes these test cases:

```rust
#[test]
fn test_delete_from_leaf() {
    // Delete key that's in a leaf node
}

#[test]
fn test_delete_from_internal() {
    // Delete key that's in an internal node
}

#[test]
fn test_delete_with_merge() {
    // Delete that triggers merge operation
}

#[test]
fn test_delete_all_keys() {
    // Delete all keys, leaving empty tree
}

#[test]
fn test_delete_nonexistent_key() {
    // Try to delete key that doesn't exist
}
```

---

## Summary

B-tree deletion maintains the balanced property through:

1. **Finding the key** using binary search
2. **Handling leaf deletions** with borrow/merge
3. **Handling internal deletions** with predecessor/successor replacement
4. **Rebalancing** to maintain minimum key requirements
5. **Reducing tree height** when root becomes empty

The algorithm ensures O(log n) time complexity while maintaining all B-tree invariants.

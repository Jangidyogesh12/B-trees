# B+ Trees in Database Systems: A Comprehensive Guide for Building Your Own Database in Rust

---

## Table of Contents

1. [Why B-Trees for Databases](#1-why-b-trees-for-databases)
2. [Disk Page Concept](#2-disk-page-concept)
3. [B+ Tree Nodes Map to Pages](#3-b-tree-nodes-map-to-pages)
4. [Buffer Pool Manager](#4-buffer-pool-manager)
5. [B+ Tree Page Traversal Step-by-Step](#5-b-tree-page-traversal-step-by-step)
6. [Search Traversal](#6-search-traversal)
7. [Insert Traversal](#7-insert-traversal)
8. [Delete Traversal](#8-delete-traversal)
9. [Range Scan Using Leaf Linked List](#9-range-scan-using-leaf-linked-list)
10. [Page Splits in Detail](#10-page-splits-in-detail)
11. [How Real Databases Do It](#11-how-real-databases-do-it)
12. [Implementation Skeleton in Rust](#12-implementation-skeleton-in-rust)
13. [Concurrency Considerations](#13-concurrency-considerations)
14. [WAL Integration with B-Tree Operations](#14-wal-integration-with-b-tree-operations)
15. [Practical Tips for Building Your Own Database](#15-practical-tips-for-building-your-own-database)

---

## 1. Why B-Trees for Databases

Databases persist data on disk. Disk I/O is the single most expensive operation in a database -- orders of magnitude slower than memory access. A single random disk read takes ~10ms (HDD) or ~0.1ms (SSD), while a memory access takes ~100ns. Every design decision in a database storage engine revolves around minimizing disk I/O.

### The Problem with Other Data Structures

```
Binary Search Tree (BST):
                    50
                   /  \
                 25    75
                / \   / \
              10  30 60  90

Problem: A tree with 1 billion keys has height ~30.
         Each level = 1 disk I/O = 30 I/Os per operation.
         Unacceptable.
```

```
Hash Index:
    hash(key) -> page_id

Problem: No range queries. Can't find "all keys between 100 and 200".
         No ordered traversal. Only point lookups.
```

### Why B+ Trees Win

B+ trees solve both problems simultaneously:

```
                        [   50   |   100   ]                    <- Root (1 page read)
                       /    |     |     \
              [20|35]      [60|80]      [120|150]               <- Internal (1 page read)
             / |  | \      / |  | \      / |  | \
            v  v  v  v    v  v  v  v    v  v  v  v
           Leaves (data)  Leaves (data)  Leaves (data)         <- Leaf (1 page read)

    Height for 1 billion keys with branching factor 500: ~4 levels
    Total I/O for any operation: 3-4 page reads. DONE.
```

**Key properties that make B+ trees ideal for databases:**

| Property | Why It Matters |
|----------|----------------|
| **High branching factor** | Each node holds hundreds of keys. Tree is shallow. Few disk I/Os. |
| **Balanced** | Every leaf is at the same depth. Guaranteed O(log n) performance. |
| **Ordered keys** | Enables range scans, order-by, and prefix lookups. |
| **All data in leaves** | Internal nodes are pure routing -- they fit more keys per page. |
| **Leaf linked list** | Sequential access without climbing back up the tree. |
| **Disk-friendly** | Each node maps to exactly one disk page. Read/write = one I/O. |

### Disk I/O Comparison

```
Operation: Find a specific key in a 1 TB database (1 billion rows)

Binary Tree:
    Height = ~30
    Each node might span a different page
    Worst case: 30 random disk reads = 30 * 10ms = 300ms

B+ Tree (page size 4KB, key size 8B, pointer size 8B):
    Branching factor = 4096 / (8 + 8) = 256
    Height = log256(1,000,000,000) = ~4
    Each node = 1 page
    Worst case: 4 random disk reads = 4 * 10ms = 40ms

    On SSD: 4 * 0.1ms = 0.4ms
```

---

## 2. Disk Page Concept

### What Is a Page?

A **page** is the fundamental unit of I/O between disk and memory. The database never reads or writes individual rows -- it reads and writes entire pages. Think of a page as a fixed-size block of bytes.

```
    DISK                                          MEMORY (Buffer Pool)
    +---+---+---+---+---+---+---+---+       +---------------------------+
    | P0| P1| P2| P3| P4| P5| P6| P7| ----> | Page Frame (fixed slot)   |
    +---+---+---+---+---+---+---+---+       |                           |
                                              |  [Actual page bytes here] |
                                              |                           |
                                              +---------------------------+
```

### Page Sizes

Different databases use different page sizes. The choice depends on the workload:

```
+----------+------------+----------------------------------+
| Page Size | Used By    | Trade-off                        |
+----------+------------+----------------------------------+
| 4 KB     | SQLite     | Matches OS page size.            |
|          |            | Good for small rows.             |
|          |            | More levels in B+ tree.          |
+----------+------------+----------------------------------+
| 8 KB     | PostgreSQL | Balanced for most workloads.     |
|          |            | Good default choice.             |
+----------+------------+----------------------------------+
| 16 KB    | InnoDB     | Fewer tree levels.               |
|          | (MySQL)    | Better for large rows.           |
|          |            | Wasted space if rows are small.  |
+----------+------------+----------------------------------+
| 64 KB    | Some OLAP  | Maximum throughput for           |
|          | systems    | sequential scans.                |
+----------+------------+----------------------------------+
```

### Page Layout

Every page has a header, and the internal layout depends on the page type (data page, internal node, leaf node). Here is a general-purpose data page layout:

```
+------------------------------------------------------------------+
| PAGE HEADER (fixed size, e.g., 24-32 bytes)                       |
+------------------------------------------------------------------+
| Page Type | Page ID | LSN | Free Space Ptr | Num Records | ...   |
+------------------------------------------------------------------+
|                     SLOT ARRAY (grows downward)                   |
|  Slot[0] -> record at offset 120                                 |
|  Slot[1] -> record at offset 96                                  |
|  Slot[2] -> record at offset 72                                  |
|  ...                                                              |
+------------------------------------------------------------------+
|                                                                  |
|                     FREE SPACE (unallocated)                     |
|                                                                  |
+------------------------------------------------------------------+
|                     RECORDS (grows upward from bottom)            |
|  Record 2: [key=300, value="hello"]                             |
|  Record 1: [key=100, value="world"]                             |
|  Record 0: [key=50,  value="foo"]                               |
+------------------------------------------------------------------+
```

**Why a slot array?** When records are deleted, you get holes. Without a slot array, you'd need to compact records on every delete. The slot array acts as an indirection layer -- records can be anywhere on the page, and the slot array maps logical index to physical offset.

```
With slot array (deletion is cheap):

  Slot[0] -> Record A (offset 120)
  Slot[1] -> [DELETED - gap]
  Slot[2] -> Record C (offset 72)

  No compaction needed. Free space grows naturally.
```

### Page Size Calculation Example

```
Page size: 8192 bytes (8 KB)
Header: 32 bytes
Slot entry: 2 bytes (offset as u16)
Record: 64 bytes average

Usable space: 8192 - 32 = 8160 bytes
Max records per page: 8160 / 64 = 127 records (approximately)
Slot array overhead: 127 * 2 = 254 bytes
Actual usable: 8160 - 254 = 7906 bytes for records
```

---

## 3. B+ Tree Nodes Map to Pages

In a disk-based B+ tree, **one node = one page**. This is the critical mapping. Instead of using memory pointers (which are meaningless across process restarts), we use **page IDs** (stable identifiers that map to disk locations).

### Pointer-Based B+ Tree (In-Memory)

```
    struct Node {
        keys: Vec<i64>,
        children: Vec<Box<Node>>,    // <-- memory pointers, gone on crash
    }
```

### Page-Based B+ Tree (Disk)

```
    struct BPlusNode {
        page_id: PageId,              // <-- stable disk identifier
        keys: Vec<i64>,
        children: Vec<PageId>,        // <-- page IDs, persisted to disk
    }
```

### The Mapping

```
B+ Tree Logical View:                    Physical Disk Layout:
                                         
        [30 | 60]                        Page 7:  [30 | 60] (internal)
       /    |    \                       Page 2:  [10|20] (leaf)
  [10|20] [40|50] [70|80]              Page 3:  [40|50] (leaf)
    v       v       v                   Page 4:  [70|80] (leaf)
  Data    Data    Data                  Page 5:  [data for 10|20]
                                        Page 6:  [data for 40|50]
                                        
Internal node has:
  keys: [30, 60]
  children: [PageId(2), PageId(3), PageId(4)]

Leaf node has:
  keys: [10, 20]
  values: [rid_1, rid_2]
  next: PageId(5)  -- pointer to next leaf
```

### Why Page IDs Instead of Pointers

```
Pointer:  0x7f8b3c004a20  -- valid only in current process memory
PageId:   42              -- valid forever, maps to disk offset 42 * PAGE_SIZE

To convert PageId to disk offset:
    disk_offset = page_id * PAGE_SIZE
    seek to disk_offset, read PAGE_SIZE bytes -> you have your page

To convert PageId to buffer pool frame:
    frame = buffer_pool.get(page_id)
    frame.data -> the actual page bytes in memory
```

### Node Types Mapped to Pages

```
+-------------------------------------------------------------------+
| INTERNAL NODE PAGE                                                  |
+-------------------------------------------------------------------+
| Header: type=INTERNAL, page_id=7, num_keys=2                      |
+-------------------------------------------------------------------+
| keys: [30, 60]                                                     |
+-------------------------------------------------------------------+
| children: [PageId(2), PageId(3), PageId(4)]                        |
|          (always num_keys + 1 children)                            |
+-------------------------------------------------------------------+


+-------------------------------------------------------------------+
| LEAF NODE PAGE                                                      |
+-------------------------------------------------------------------+
| Header: type=LEAF, page_id=2, num_keys=2, next=PageId(5)          |
+-------------------------------------------------------------------+
| keys: [10, 20]                                                     |
+-------------------------------------------------------------------+
| values: [RecordId(page=9, slot=3), RecordId(page=9, slot=7)]      |
+-------------------------------------------------------------------+
```

---

## 4. Buffer Pool Manager

The buffer pool is the cache between the B+ tree (which thinks in pages) and the disk (which thinks in bytes). It keeps frequently accessed pages in memory and evicts cold pages when memory is full.

### Architecture

```
    B+ Tree Code
        |
        | get_page(PageId) / put_page(PageId, page)
        v
    +------------------------------------------+
    |         BUFFER POOL MANAGER              |
    |                                          |
    |  PageId -> Frame mapping (hash table)   |
    |  [PageId 2 -> Frame 0]                  |
    |  [PageId 7 -> Frame 3]                  |
    |  [PageId 4 -> Frame 5]                  |
    |                                          |
    |  Frame 0: [page data, pin_count=3, dirty]|
    |  Frame 1: [page data, pin_count=0, clean]|
    |  Frame 2: [page data, pin_count=1, clean]|
    |  ...                                     |
    |                                          |
    |  LRU Replacer: eviction policy           |
    +------------------------------------------+
        |                    |
        v                    v
    DISK (files)         MEMORY (heap)
```

### Core Concepts

**Frame**: A slot in the buffer pool that holds one page. Fixed number of frames = fixed memory usage.

**Pin count**: How many threads are currently using this page. A page with pin_count > 0 cannot be evicted.

**Dirty flag**: Modified since last disk write. Must be flushed before eviction.

### LRU (Least Recently Used) Eviction

```
Buffer Pool (4 frames), accessing pages in order: 2, 3, 4, 2, 5

Step 1: get_page(2) -> MISS -> read from disk -> pin=1
  LRU order: [2]

Step 2: get_page(3) -> MISS -> read from disk -> pin=1
  LRU order: [2, 3]

Step 3: get_page(4) -> MISS -> read from disk -> pin=1
  LRU order: [2, 3, 4]

Step 4: get_page(2) -> HIT -> pin=2
  LRU order: [3, 4, 2]    (2 moved to back = most recently used)

Step 5: get_page(5) -> MISS -> need to evict
  Evict front of LRU: Page 3 (least recently used)
  LRU order: [4, 2, 5]

Step 6: get_page(3) -> MISS -> read from disk again (was evicted!)
```

### LRU Variants

Simple LRU has a problem: a sequential scan (reading every page once) pollutes the cache, evicting hot pages. Solutions:

- **LRU-K**: Track the K-th most recent access time. Only promote to "hot" after K accesses.
- **Clock (Second Chance)**: Circular buffer with reference bits. Cheaper than full LRU.
- **ARC (Adaptive Replacement Cache)**: Balances between recency and frequency. Used in some production databases.

### Rust Implementation Sketch

```rust
use std::collections::HashMap;

type PageId = u64;

const PAGE_SIZE: usize = 8192;

#[derive(Clone)]
struct Page {
    data: [u8; PAGE_SIZE],
}

struct Frame {
    page: Page,
    page_id: PageId,
    pin_count: u32,
    is_dirty: bool,
}

struct LRUReplacer {
    order: Vec<PageId>,
}

impl LRUReplacer {
    fn new() -> Self {
        Self { order: Vec::new() }
    }

    fn record_access(&mut self, page_id: PageId) {
        self.order.retain(|&id| id != page_id);
        self.order.push(page_id);
    }

    fn evict(&mut self) -> Option<PageId> {
        self.order.first().copied().map(|id| {
            self.order.remove(0);
            id
        })
    }
}

struct BufferPoolManager {
    pool: Vec<Frame>,
    page_table: HashMap<PageId, usize>,  // page_id -> frame index
    replacer: LRUReplacer,
    num_frames: usize,
}

impl BufferPoolManager {
    fn new(num_frames: usize) -> Self {
        let pool = (0..num_frames)
            .map(|i| Frame {
                page: Page { data: [0u8; PAGE_SIZE] },
                page_id: 0,
                pin_count: 0,
                is_dirty: false,
            })
            .collect();

        Self {
            pool,
            page_table: HashMap::new(),
            replacer: LRUReplacer::new(),
            num_frames,
        }
    }

    fn get_page(&mut self, page_id: PageId) -> Option<&mut Frame> {
        // Check if page is already in pool
        if let Some(&frame_idx) = self.page_table.get(&page_id) {
            self.pool[frame_idx].pin_count += 1;
            self.replacer.record_access(page_id);
            return Some(&mut self.pool[frame_idx]);
        }

        // Page not in pool -- need to fetch from disk
        let frame_idx = self.find_victim_frame()?;
        let victim = &mut self.pool[frame_idx];

        // Flush dirty victim before overwriting
        if victim.is_dirty {
            self.flush_page(victim.page_id);
        }

        // Remove old mapping
        self.page_table.remove(&victim.page_id);

        // Load new page from disk
        victim.page_id = page_id;
        victim.pin_count = 1;
        victim.is_dirty = false;
        self.read_page_from_disk(page_id, &mut victim.page);

        self.page_table.insert(page_id, frame_idx);
        self.replacer.record_access(page_id);

        Some(&mut self.pool[frame_idx])
    }

    fn find_victim_frame(&self) -> Option<usize> {
        // Find frame with pin_count == 0 using LRU
        for &page_id in &self.replacer.order {
            if let Some(&idx) = self.page_table.get(&page_id) {
                if self.pool[idx].pin_count == 0 {
                    return Some(idx);
                }
            }
        }
        None
    }

    fn flush_page(&self, page_id: PageId) {
        // In a real implementation: seek to page_id * PAGE_SIZE, write data
        println!("Flushing page {} to disk", page_id);
    }

    fn read_page_from_disk(&self, page_id: PageId, page: &mut Page) {
        // In a real implementation: seek to page_id * PAGE_SIZE, read PAGE_SIZE bytes
        println!("Reading page {} from disk", page_id);
    }

    fn unpin_page(&mut self, page_id: PageId, is_dirty: bool) {
        if let Some(&idx) = self.page_table.get(&page_id) {
            let frame = &mut self.pool[idx];
            if frame.pin_count > 0 {
                frame.pin_count -= 1;
            }
            if is_dirty {
                frame.is_dirty = true;
            }
        }
    }

    fn flush_all_pages(&mut self) {
        for frame in &self.pool {
            if frame.is_dirty {
                self.flush_page(frame.page_id);
            }
        }
    }
}
```

### Important: Pinning Protocol

```
1.  frame = bpm.get_page(page_id)      // pin_count += 1
2.  ... read/write frame.data ...       // do work
3.  bpm.unpin_page(page_id, is_dirty)  // pin_count -= 1

    NEVER hold a reference to a frame after unpinning.
    The frame may be evicted and reused at any time after unpin.
```

---

## 5. B+ Tree Page Traversal Step-by-Step

Let us trace through a concrete B+ tree with actual page IDs and show how traversal works.

### Example B+ Tree

```
Page 1 (ROOT, internal):
    keys: [50, 100]
    children: [PageId(2), PageId(3), PageId(4)]

Page 2 (internal):
    keys: [20, 35]
    children: [PageId(5), PageId(6), PageId(7)]

Page 3 (internal):
    keys: [65, 80]
    children: [PageId(8), PageId(9), PageId(10)]

Page 4 (internal):
    keys: [110, 130]
    children: [PageId(11), PageId(12), PageId(13)]

Pages 5-13 (LEAVES):
    Page 5:  keys=[5,10,15]      next=PageId(6)
    Page 6:  keys=[20,25,30]     next=PageId(7)
    Page 7:  keys=[35,40,45]     next=PageId(8)
    Page 8:  keys=[50,55,60]     next=PageId(9)
    Page 9:  keys=[65,70,75]     next=PageId(10)
    Page 10: keys=[80,85,90]     next=PageId(11)
    Page 11: keys=[100,105,108]  next=PageId(12)
    Page 12: keys=[110,115,120]  next=PageId(13)
    Page 13: keys=[130,140,150]  next=NULL
```

```
Visual layout:

                    Page 1
               [  50  |  100  ]
              /       |        \
         Page 2     Page 3     Page 4
      [20|35]     [65|80]   [110|130]
      /  |  \     /  |  \   /   |   \
    P5  P6  P7  P8  P9 P10 P11 P12 P13
```

### Traversal Rules

At each internal node, binary search the keys to determine which child pointer to follow:

```
if key < keys[0]:           follow children[0]
if keys[0] <= key < keys[1]: follow children[1]
if keys[1] <= key:           follow children[2]

Generalized: find the rightmost key that is <= target, follow the child after it.
```

### ASCII Animation: Traversal to Find Key 72

```
Step 1: Load Page 1 (ROOT) from disk
        Disk I/O count: 1
        
        Page 1: [50, 100]
        72 >= 50 and 72 < 100 -> follow children[1] -> Page 3

Step 2: Load Page 3 from disk
        Disk I/O count: 2
        
        Page 3: [65, 80]
        72 >= 65 and 72 < 80 -> follow children[1] -> Page 9

Step 3: Load Page 9 from disk (leaf)
        Disk I/O count: 3
        
        Page 9: keys=[65, 70, 75]
        Binary search for 72 -> found between 70 and 75 -> not exact match
        Return: not found (or the position where it would be)

Total disk I/Os: 3 page reads.
```

---

## 6. Search Traversal

### Complete Search Algorithm

```
function search(tree, target_key):
    // Step 1: Read root page
    current_page_id = tree.root_page_id
    io_count = 0
    
    while current_page is INTERNAL:
        page = buffer_pool.get_page(current_page_id)    // +1 I/O
        io_count += 1
        
        // Binary search keys to find correct child
        child_index = 0
        for i in 0..page.num_keys:
            if target_key >= page.keys[i]:
                child_index = i + 1
        
        next_page_id = page.children[child_index]
        buffer_pool.unpin_page(current_page_id, false)
        current_page_id = next_page_id
    
    // current_page is now a LEAF
    page = buffer_pool.get_page(current_page_id)        // +1 I/O
    io_count += 1
    
    // Binary search within leaf
    result = binary_search(page.keys, target_key)
    
    buffer_pool.unpin_page(current_page_id, false)
    
    return (result, io_count)
```

### Disk I/O Analysis

```
Branching factor (b): number of children per internal node
    b = floor(PAGE_SIZE / (key_size + pointer_size))
    
    For PAGE_SIZE = 8192, key_size = 8, pointer_size = 8:
    b = floor(8192 / 16) = 512

Tree height for N keys: h = ceil(log_b(N))

    N = 1,000,000:     h = ceil(log512(1,000,000)) = ceil(2.58) = 3
    N = 1,000,000,000: h = ceil(log512(1,000,000,000)) = ceil(3.87) = 4

Search I/Os = h (one page read per level)

    1 million keys:    3 I/Os
    1 billion keys:    4 I/Os
    
Compare to binary search on sorted array:
    1 billion keys:    30 I/Os (log2(1,000,000,000))
```

### Example: Searching for Key 85

```
    Load Page 1 (root):  [50 | 100]
                         85 >= 50, 85 < 100 -> go to Page 3
                          I/O: 1

    Load Page 3:         [65 | 80]
                         85 >= 65, 85 >= 80 -> go to Page 10
                          I/O: 2

    Load Page 10 (leaf): keys=[80, 85, 90]
                         Binary search: found 85 at index 1
                          I/O: 3

    Return: (key=85, record_id=<page=99, slot=5>)
    Total I/Os: 3
```

---

## 7. Insert Traversal

### Algorithm

```
function insert(tree, key, value):
    // Step 1: Find the correct leaf
    leaf_page_id = find_leaf(tree, key)
    leaf = buffer_pool.get_page(leaf_page_id)
    
    // Step 2: Insert into leaf
    insert_into_sorted(leaf.keys, key)
    insert_into_sorted(leaf.values, value)
    leaf.is_dirty = true
    
    // Step 3: Check if leaf overflows
    if leaf.num_keys <= MAX_KEYS_PER_PAGE:
        // No split needed. Done.
        unpin(leaf_page_id, dirty=true)
        return
    
    // Step 4: Split the leaf
    new_page_id = allocate_new_page()
    new_leaf = buffer_pool.get_page(new_page_id)
    
    // Copy upper half to new page
    mid = leaf.num_keys / 2
    new_leaf.keys = leaf.keys[mid..]
    new_leaf.values = leaf.values[mid..]
    leaf.keys = leaf.keys[..mid]
    leaf.values = leaf.values[..mid]
    
    // Update leaf linked list
    new_leaf.next = leaf.next
    leaf.next = new_page_id
    
    // Step 5: Push median key up to parent
    push_up_key(tree, parent_page_id, new_leaf.keys[0], new_page_id)
```

### ASCII Animation: Inserting Key 57

Starting tree:

```
                    Page 1
               [  50  |  100  ]
              /       |        \
         Page 2     Page 3     Page 4
      [20|35]     [65|80]   [110|130]
      /  |  \     /  |  \   /   |   \
    P5  P6  P7  P8  P9 P10 P11 P12 P13
```

Insert 57 -> goes to Page 8 (leaf): keys=[50,55,60]

```
Step 1: Load Page 1 -> 57 >= 50, 57 < 100 -> go to Page 3      (I/O: 1)
Step 2: Load Page 3 -> 57 < 65 -> go to Page 8                  (I/O: 2)
Step 3: Load Page 8 -> insert 57                                 (I/O: 3)
        Page 8 now: keys=[50, 55, 57, 60]
```

Page 8 fits within MAX_KEYS. No split needed. DONE.

But what if Page 8 was full and we needed to split?

```
假设 Page 8 was full: keys=[50, 55, 60]  (capacity = 3 for this example)

After inserting 57: keys=[50, 55, 57, 60]  -- OVERFLOW!
```

### Splitting the Leaf (detailed in Section 10)

```
BEFORE SPLIT:
    Page 8: [50, 55, 57, 60]   -- overflow
    
AFTER SPLIT:
    Page 8:  [50, 55]          -- lower half stays
    Page 14: [57, 60]          -- upper half moves to new page
    
    Leaf linked list: ... -> Page 7 -> Page 8 -> Page 14 -> Page 9 -> ...
    
    Push up key 57 to parent (Page 3)
    
AFTER PUSH UP:
    Page 3: [57, 65, 80]       -- key 57 inserted
    Page 3 children: [Page 8, Page 14, Page 9, Page 10]
```

### Inserting into Internal Node (Cascading Split)

If the parent internal node also overflows, it splits too, potentially all the way to the root. If the root splits, a new root is created and the tree grows one level taller.

```
    Root splits -> new root created -> tree height increases by 1
    This is the ONLY way a B+ tree grows taller.
```

---

## 8. Delete Traversal

### Algorithm

```
function delete(tree, key):
    // Step 1: Find the leaf containing key
    leaf_page_id = find_leaf(tree, key)
    leaf = buffer_pool.get_page(leaf_page_id)
    
    // Step 2: Remove key from leaf
    remove_from_sorted(leaf.keys, key)
    remove_from_sorted(leaf.values, corresponding_value)
    leaf.is_dirty = true
    
    // Step 3: Check if leaf underflows
    if leaf.num_keys >= MIN_KEYS_PER_PAGE:
        // No underflow. Done.
        unpin(leaf_page_id, dirty=true)
        return
    
    // Step 4: Handle underflow
    // Try to borrow from left sibling
    if can_borrow_from_left_sibling(leaf):
        borrow_from_left(leaf, left_sibling)
        return
    
    // Try to borrow from right sibling
    if can_borrow_from_right_sibling(leaf):
        borrow_from_right(leaf, right_sibling)
        return
    
    // Must merge
    merge(leaf, sibling)
    // May cause parent to underflow -> recurse up
```

### Borrowing (Redistribution)

```
BEFORE: Left sibling is overfull, current node is underfull

    Left Sibling (Page 7): [35, 40, 45]    Current (Page 8): [50]
                                   |                            |
                                   v                            v
                                 data                         data

AFTER borrowing from right:

    Left Sibling (Page 7): [35, 40]        Current (Page 8): [45, 50]
    
    Parent key updated: 45 (was 50)
```

### Merging

```
BEFORE: Both siblings are at minimum, can't borrow

    Left (Page 7): [35, 40]    Right (Page 8): [50, 55]
    Parent key: 50
    
AFTER merge:

    Merged (Page 7): [35, 40, 50, 55]
    Page 8 is freed
    Parent removes key 50 and child pointer to Page 8
    
    If parent underflows -> repeat at parent level
```

### ASCII Animation: Deleting Key 15

```
Starting state:
    Page 5: keys=[5, 10, 15]  (has 3 keys, min is 2)

Step 1: Load Page 1 -> 15 < 50 -> go to Page 2              (I/O: 1)
Step 2: Load Page 2 -> 15 < 20 -> go to Page 5              (I/O: 2)
Step 3: Load Page 5 -> remove 15                             (I/O: 3)
        Page 5: keys=[5, 10]
        
    num_keys (2) >= MIN_KEYS (2) -> no underflow. DONE.
```

### What If Deleting Key 5 Causes Underflow?

```
    Page 5: keys=[5, 10]  -> remove 5 -> keys=[10]
    
    num_keys (1) < MIN_KEYS (2) -> UNDERFLOW
    
    Check siblings:
        Page 6: keys=[20, 25, 30]  -> has 3 keys, can lend 1
    
    Borrow from Page 6:
        Move 20 from Page 6 to Page 5
        Page 5: [10, 20]
        Page 6: [25, 30]
        Parent key updated: 25 (was 20)
```

---

## 9. Range Scan Using Leaf Linked List

### Why Leaf Linked List Is Efficient

The B+ tree's killer feature for databases is that **all leaf pages are connected in a doubly (or singly) linked list**. This means once you find the starting point of a range scan, you can traverse sequentially through leaf pages WITHOUT going back up to internal nodes.

```
Leaf linked list:

    Page 5 <-> Page 6 <-> Page 7 <-> Page 8 <-> Page 9 <-> Page 10 <-> ...
    [5,10]   [20,25]   [35,40]   [50,55]   [65,70]   [80,85]
```

### Range Scan: Find All Keys Between 25 and 65

```
Step 1: Search for starting key (25)
        Root -> Page 2 -> Page 6 (leaf)
        Found key 25 at index 0 in Page 6
        I/Os: 3 (same as point search)
        
Step 2: Sequential scan through leaf linked list
        Read Page 6: output [25, 30]           (I/O: 1)
        Follow next pointer -> Page 7
        Read Page 7: output [35, 40]           (I/O: 2)
        Follow next pointer -> Page 8
        Read Page 8: output [50, 55]           (I/O: 3)
        Follow next pointer -> Page 9
        Read Page 9: output [65]               (I/O: 4) -- 65 <= 65, include it
        Follow next pointer -> Page 10
        Read Page 10: keys[0] = 80 > 65 -> STOP
        
Total I/Os: 3 (search) + 4 (scan) = 7
Results: [25, 30, 35, 40, 50, 55, 65]
```

### Comparison Without Linked List (Hypothetical)

```
Without leaf linked list, for each result you'd need:
    Start at root, traverse down to find each key.
    
    For range [25, 65] with 7 results:
    7 * 3 I/Os = 21 I/Os (7 separate searches from root)
    
With leaf linked list:
    3 I/Os (initial search) + 4 I/Os (sequential scan) = 7 I/Os
    
    And sequential I/Os are much faster than random I/Os:
    HDD:  sequential = 200 MB/s, random = 2 MB/s  (100x difference)
    SSD:  sequential = 2 GB/s,   random = 0.5 GB/s (4x difference)
```

### Predecessor/Successor Queries

The linked list also enables efficient:
- `SELECT * FROM t WHERE key > 100 ORDER BY key LIMIT 10` -- find 100 in leaf, scan 10 records forward
- `SELECT * FROM t WHERE key < 50 ORDER BY key DESC LIMIT 10` -- find 50 in leaf, scan backward
- `SELECT COUNT(*) FROM t WHERE key BETWEEN 100 AND 200` -- range scan, count as you go

---

## 10. Page Splits in Detail

Page splits are the most complex operation in a B+ tree. This is where most implementation bugs hide.

### Leaf Page Split

```
BEFORE SPLIT (leaf page overflow):

    Page 8: [50, 55, 57, 60]    <- 4 keys, capacity is 3
    
    Parent (Page 3):
        keys: [65, 80]
        children: [Page 8, Page 9, Page 10]

    Parent must know about Page 8 for key range [57, 65)
    
            Page 3
        [  65  |  80  ]
       /       |       \
    Page 8   Page 9   Page 10
    [50,55,57,60]  [65,70,75]  [80,85,90]


STEP 1: Allocate new page (Page 14)
STEP 2: Split keys in half

    Page 8:  [50, 55]        (lower half)
    Page 14: [57, 60]        (upper half, new page)

STEP 3: Update leaf linked list

    ... -> Page 7 -> Page 8 -> Page 14 -> Page 9 -> ...
    
    Page 14.next = Page 8.next (was Page 9)
    Page 8.next = Page 14

STEP 4: Insert separator key into parent

    The separator key is the smallest key in the new right page (57).
    Insert 57 into Page 3 between 65 and 80.
    
    Page 3:
        keys: [57, 65, 80]
        children: [Page 8, Page 14, Page 9, Page 10]

    BEFORE:                         AFTER:
         Page 3                          Page 3
     [  65  |  80  ]              [57 | 65 | 80]
    /       |       \             /    |    |    \
  P8       P9      P10          P8   P14   P9   P10
  [...]    [...]   [...]        [...] [...] [...] [...]
```

### Internal Page Split

Internal page splits work differently -- the median key is **pushed up** to the parent (not copied).

```
BEFORE SPLIT (internal page overflow):

    Page 3: keys=[57, 65, 72, 80]    <- overflow (4 keys, max 3)
            children=[P8, P14, P9, P10, P11]
    
         Page 3
    [57|65|72|80]
    / | | |  \
   P8 P14 P9 P10 P11


STEP 1: Median key is 65 (middle key)
STEP 2: Push median up to parent, split around it

    Page 3:  [57]              (left of median)
    Page 15: [72, 80]          (right of median, new page)
    Page 3 children: [P8, P14]
    Page 15 children: [P9, P10, P11]

STEP 3: Insert pushed-up key (65) into parent

    Parent (Page 1): [50, 65, 100]   <- 65 inserted
    
    BEFORE:                         AFTER:
        Page 1                          Page 1
    [  50  |  100  ]              [50 | 65 | 100]
   /       |        \            /      |      |   \
  P2      P3       P4          P2     P3     P15   P4
```

**Key difference:**
- Leaf split: copy median to parent (median stays in left leaf AND appears in parent)
- Internal split: push median to parent (median is removed from the splitting node)

### Split Cascade to Root

```
If Page 1 (root) overflows:
    
    BEFORE:         [50 | 65 | 100 | 150]    <- overflow!
                   /    |     |      \
                 P2    P3    P15     P4

    Median: 65 or 100 (depending on implementation)
    
    Push 100 up -> create new root
    
    NEW ROOT (Page 16): [100]
                       /       \
               Page 1           Page 15
            [50 | 65]        [150]
            /   |   \           |   \
          P2   P3   P4       ...   ...
          
    Tree height: 3 -> 4
    This is the ONLY time the tree grows.
```

---

## 11. How Real Databases Do It

### PostgreSQL: B-Tree Indexes

```
PostgreSQL uses a modified B-tree called "nbtree" (near-B-tree):

Key differences:
- Uses "items" (HeapTupleIds) stored in leaf pages
- Internal pages store "high keys" for navigation
- Pages have a "line pointer" array (like slot array)
- Uses a "leftmost" pointer that is implicit
- Supports duplicate keys via TID (tuple ID) comparison
- Page size: 8 KB (compile-time constant)

PostgreSQL page layout:
+------------------+
| PageHeaderData   |  (24 bytes)
| pd_lsn           |  (LSN for WAL)
| pd_checksum      |
| pd_lower         |  (offset to start of free space)
| pd_upper         |  (offset to end of free space)
| pd_special       |  (offset to special space)
+------------------+
| ItemIdData array |  (line pointers, 4 bytes each)
+------------------+
| Free space       |
+------------------+
| Tuples           |  (grows backward from pd_upper)
+------------------+
| Special space    |  (for B-tree metadata)
+------------------+
```

### InnoDB (MySQL): Clustered Index

```
InnoDB uses a clustered index where the PRIMARY KEY index IS the data:

Clustered Index:
    Leaf pages contain the actual row data, not just pointers.
    The table IS the primary key B+ tree.
    
Secondary Indexes:
    Leaf pages contain (indexed_column_value, primary_key_value)
    To get the full row: secondary index lookup -> primary key lookup
    
InnoDB page (16 KB):
+-------------------+
| File Header       |  (38 bytes: page type, LSN, etc.)
+-------------------+
| Page Header       |  (56 bytes: num records, etc.)
+-------------------+
| Infimum/Supremum  |  (virtual min/max records)
+-------------------+
| User Records      |  (actual data, sorted by primary key)
+-------------------+
| Free Space        |
+-------------------+
| Page Directory    |  (slot array, 2 bytes per slot)
+-------------------+
| File Trailer      |  (8 bytes: checksum)
+-------------------+

InnoDB uses "page split" similar to our discussion, but with
additional complexity for maintaining the clustered index order.
```

### LMDB: Lightning Memory-Mapped Database

```
LMDB (used by OpenLDAP, many other projects):

Key design:
- Memory-mapped files (mmap) -- OS handles caching
- Copy-on-write B+ tree (never overwrites live pages)
- Single writer, lock-free readers (MVCC via versioning)
- No buffer pool manager -- relies on OS page cache
- Branching factor: depends on key size, typically 200-500

    LMDB tree:
    
    Each page is a fixed-size block (4 KB default).
    Pages are referenced by file offset (no page ID abstraction needed
    because mmap gives you a direct pointer).
    
    Readers get a consistent snapshot by pinning a meta page.
    Writers create a new version of the tree (copy-on-write).
```

### SQLite: B-Tree Pages

```
SQLite uses a B-tree where both internal and leaf nodes are stored
in pages of a single database file.

Page types:
- 10: B-tree leaf page (stores data)
- 13: B-tree interior page (stores child pointers)

SQLite has TWO separate B-trees per table:
1. A B-tree for the table (rowid -> row data)
2. An index B-tree (indexed_value -> rowid)

Page size: 512 bytes to 65536 bytes (configurable, default 4096)

SQLite page header (leaf):
+------------------+
| 0x0d (leaf flag) |  (1 byte)
| first_freebyte   |  (2 bytes)
| num_cells        |  (2 bytes)
| cell_content_area|  (2 bytes)
| fragmented_bytes |  (1 byte)
+------------------+
| Cell Pointer Array|  (2 bytes each, grows from start)
+------------------+
| Free Space       |
+------------------+
| Cell Content     |  (grows from end)
+------------------+
```

### Comparison Table

```
+----------------+------------+---------+----------+-----------+
| Feature        | PostgreSQL | InnoDB  | SQLite   | LMDB      |
+----------------+------------+---------+----------+-----------+
| Page size      | 8 KB       | 16 KB   | 4 KB     | 4 KB      |
| Key type       | Variable   | Variable| Variable | Variable  |
| Clustered?     | No (heap)  | Yes     | Partial  | Yes       |
| WAL?           | Yes        | Yes     | Yes      | No (CoW)  |
| Concurrency    | MVCC       | MVCC    | Locking  | MVCC      |
| B+ tree impl   | nbtree     | Custom  | Custom   | Custom    |
+----------------+------------+---------+----------+-----------+
```

---

## 12. Implementation Skeleton in Rust

### Core Types

```rust
use std::collections::HashMap;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Page Identity
// ---------------------------------------------------------------------------

/// Unique identifier for a page on disk.
/// PageId(0) is reserved for the meta/root page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageId(pub u64);

impl PageId {
    pub fn invalid() -> Self {
        PageId(u64::MAX)
    }

    pub fn is_invalid(self) -> bool {
        self.0 == u64::MAX
    }
}

/// Record ID: uniquely identifies a row in the database.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordId {
    pub page_id: PageId,
    pub slot: u16,
}

// ---------------------------------------------------------------------------
// Page Types
// ---------------------------------------------------------------------------

pub const PAGE_SIZE: usize = 8192;

/// Every page starts with this header.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PageHeader {
    pub page_type: PageType,
    pub page_id: PageId,
    pub num_keys: u16,
    pub lsn: u64,               // Log Sequence Number (for WAL)
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    Invalid = 0,
    Internal = 1,
    Leaf = 2,
}

/// Internal node page layout (stored in a raw page).
///
/// Layout:
///   [PageHeader | child_0 | key_0 | child_1 | key_1 | ... | child_n]
///
/// Invariant: num_keys == num_children - 1
#[derive(Debug)]
pub struct InternalNode {
    pub header: PageHeader,
    /// Child page IDs. Length is always num_keys + 1.
    pub children: Vec<PageId>,
    /// Separator keys. children[i] contains keys < keys[i],
    /// children[i+1] contains keys >= keys[i].
    pub keys: Vec<i64>,
}

/// Leaf node page layout.
///
/// Layout:
///   [PageHeader | key_0 | value_0 | key_1 | value_1 | ... | next_page_id]
#[derive(Debug)]
pub struct LeafNode {
    pub header: PageHeader,
    pub keys: Vec<i64>,
    /// Values stored in the leaf. For a heap-organized table these are
    /// RecordIds. For an index they could be the indexed column value.
    pub values: Vec<RecordId>,
    /// Pointer to the next leaf (for range scans). InvalidPageId = end.
    pub next: PageId,
}

// ---------------------------------------------------------------------------
// Buffer Pool
// ---------------------------------------------------------------------------

/// A frame in the buffer pool. Holds exactly one page of data.
#[derive(Debug)]
pub struct Frame {
    pub data: Vec<u8>,
    pub page_id: PageId,
    pub pin_count: u32,
    pub is_dirty: bool,
}

/// Simple LRU eviction policy.
pub struct LRUReplacer {
    order: Vec<PageId>,
}

impl LRUReplacer {
    pub fn new() -> Self {
        Self {
            order: Vec::new(),
        }
    }

    pub fn record_access(&mut self, page_id: PageId) {
        self.order.retain(|&id| id != page_id);
        self.order.push(page_id);
    }

    pub fn evict(&mut self) -> Option<PageId> {
        if self.order.is_empty() {
            return None;
        }
        Some(self.order.remove(0))
    }

    pub fn size(&self) -> usize {
        self.order.len()
    }
}

/// The Buffer Pool Manager.
///
/// Caches disk pages in memory and mediates all access between the
/// B+ tree code and the disk layer.
pub struct BufferPoolManager {
    pool: Vec<Frame>,
    page_table: HashMap<PageId, usize>,  // page_id -> frame index
    replacer: LRUReplacer,
    num_frames: usize,
    disk_path: PathBuf,
}

impl BufferPoolManager {
    pub fn new(num_frames: usize, disk_path: PathBuf) -> Self {
        let pool: Vec<Frame> = (0..num_frames)
            .map(|_| Frame {
                data: vec![0u8; PAGE_SIZE],
                page_id: PageId::invalid(),
                pin_count: 0,
                is_dirty: false,
            })
            .collect();

        Self {
            pool,
            page_table: HashMap::new(),
            replacer: LRUReplacer::new(),
            num_frames,
            disk_path,
        }
    }

    /// Fetch a page into the buffer pool and return its frame index.
    /// If the page is already cached, returns immediately (cache hit).
    /// If not, evicts a victim page, flushes it if dirty, and loads
    /// the requested page from disk.
    pub fn get_page(&mut self, page_id: PageId) -> Result<usize, BPlusTreeError> {
        // Cache hit
        if let Some(&frame_idx) = self.page_table.get(&page_id) {
            self.pool[frame_idx].pin_count += 1;
            self.replacer.record_access(page_id);
            return Ok(frame_idx);
        }

        // Cache miss -- find a victim frame
        let frame_idx = self
            .find_victim_frame()
            .ok_or(BPlusTreeError::BufferPoolFull)?;

        let victim = &mut self.pool[frame_idx];

        // Flush dirty victim
        if victim.is_dirty {
            self.flush_page_to_disk(victim.page_id)?;
            victim.is_dirty = false;
        }

        // Evict old mapping
        if !victim.page_id.is_invalid() {
            self.page_table.remove(&victim.page_id);
        }

        // Load new page
        self.read_page_from_disk(page_id, &mut victim.data)?;
        victim.page_id = page_id;
        victim.pin_count = 1;

        self.page_table.insert(page_id, frame_idx);
        self.replacer.record_access(page_id);

        Ok(frame_idx)
    }

    /// Unpin a page. If modified, mark it dirty so it gets flushed later.
    pub fn unpin_page(&mut self, page_id: PageId, is_dirty: bool) {
        if let Some(&idx) = self.page_table.get(&page_id) {
            let frame = &mut self.pool[idx];
            if frame.pin_count > 0 {
                frame.pin_count -= 1;
            }
            if is_dirty {
                frame.is_dirty = true;
            }
        }
    }

    /// Allocate a new page and return its PageId.
    pub fn allocate_page(&mut self) -> PageId {
        // In a real implementation: maintain a free page list or
        // use a counter from the meta page.
        static mut NEXT_PAGE_ID: u64 = 1;
        let id = unsafe {
            let id = NEXT_PAGE_ID;
            NEXT_PAGE_ID += 1;
            PageId(id)
        };
        id
    }

    fn find_victim_frame(&self) -> Option<usize> {
        // Find the first unpinned page via LRU order
        for page_id in &self.replacer.order {
            if let Some(&idx) = self.page_table.get(page_id) {
                if self.pool[idx].pin_count == 0 {
                    return Some(idx);
                }
            }
        }
        None
    }

    fn flush_page_to_disk(&self, page_id: PageId) -> Result<(), BPlusTreeError> {
        if let Some(&idx) = self.page_table.get(&page_id) {
            let offset = page_id.0 as usize * PAGE_SIZE;
            // In a real implementation: use std::fs::File and pwrite
            // let mut file = std::fs::OpenOptions::new().write(true).open(&self.disk_path)?;
            // file.seek(SeekFrom::Start(offset as u64))?;
            // file.write_all(&self.pool[idx].data)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn read_page_from_disk(
        &self,
        page_id: PageId,
        buf: &mut [u8],
    ) -> Result<(), BPlusTreeError> {
        let offset = page_id.0 as usize * PAGE_SIZE;
        // In a real implementation:
        // let mut file = std::fs::File::open(&self.disk_path)?;
        // file.seek(SeekFrom::Start(offset as u64))?;
        // file.read_exact(buf)?;
        Ok(())
    }

    pub fn flush_all(&mut self) -> Result<(), BPlusTreeError> {
        for frame in &self.pool {
            if frame.is_dirty {
                self.flush_page_to_disk(frame.page_id)?;
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// B+ Tree
// ---------------------------------------------------------------------------

/// Configuration for the B+ tree.
#[derive(Debug, Clone)]
pub struct BPlusTreeConfig {
    pub page_size: usize,
    /// Maximum keys per internal node.
    /// Typically: (page_size - header_size) / (key_size + pointer_size)
    pub max_internal_keys: usize,
    /// Maximum keys per leaf node.
    /// Typically: (page_size - header_size - next_ptr_size) / (key_size + value_size)
    pub max_leaf_keys: usize,
    /// Minimum keys (half of max, rounded up).
    pub min_keys: usize,
}

impl BPlusTreeConfig {
    pub fn default_8kb() -> Self {
        let header_size = std::mem::size_of::<PageHeader>();
        let max_internal = (PAGE_SIZE - header_size) / (8 + 8); // key=8B, ptr=8B
        let max_leaf = (PAGE_SIZE - header_size - 8) / (8 + 16); // key=8B, val=16B(RecordId)
        let min = max_internal / 2;
        Self {
            page_size: PAGE_SIZE,
            max_internal_keys: max_internal,
            max_leaf_keys: max_leaf,
            min_keys: min,
        }
    }
}

/// Errors that can occur during B+ tree operations.
#[derive(Debug)]
pub enum BPlusTreeError {
    BufferPoolFull,
    PageNotFound(PageId),
    KeyNotFound(i64),
    DiskError(std::io::Error),
    TreeCorrupted(String),
}

impl From<std::io::Error> for BPlusTreeError {
    fn from(e: std::io::Error) -> Self {
        BPlusTreeError::DiskError(e)
    }
}

/// The B+ Tree itself.
pub struct BPlusTree {
    root_page_id: PageId,
    buffer_pool: BufferPoolManager,
    config: BPlusTreeConfig,
}

impl BPlusTree {
    pub fn new(buffer_pool: BufferPoolManager, config: BPlusTreeConfig) -> Self {
        // Create root page as an empty leaf
        let mut bpm = buffer_pool;
        let root_page_id = bpm.allocate_page();
        let frame_idx = bpm.get_page(root_page_id).unwrap();

        // Initialize root as an empty leaf
        let page = &mut bpm.pool[frame_idx];
        page.data[0] = PageType::Leaf as u8;
        page.is_dirty = true;
        bpm.unpin_page(root_page_id, true);

        Self {
            root_page_id,
            buffer_pool: bpm,
            config,
        }
    }

    /// Search for a key in the B+ tree.
    /// Returns the RecordId if found, None otherwise.
    pub fn search(&mut self, key: i64) -> Result<Option<RecordId>, BPlusTreeError> {
        let leaf_page_id = self.find_leaf(key)?;
        self.search_in_leaf(leaf_page_id, key)
    }

    /// Insert a key-value pair into the B+ tree.
    pub fn insert(
        &mut self,
        key: i64,
        value: RecordId,
    ) -> Result<(), BPlusTreeError> {
        let leaf_page_id = self.find_leaf(key)?;
        self.insert_into_leaf(leaf_page_id, key, value)
    }

    /// Delete a key from the B+ tree.
    pub fn delete(&mut self, key: i64) -> Result<(), BPlusTreeError> {
        let leaf_page_id = self.find_leaf(key)?;
        self.delete_from_leaf(leaf_page_id, key)
    }

    /// Range scan: return all (key, value) pairs where start <= key <= end.
    pub fn range_scan(
        &mut self,
        start: i64,
        end: i64,
    ) -> Result<Vec<(i64, RecordId)>, BPlusTreeError> {
        let mut results = Vec::new();

        // Find the leaf containing the start key
        let mut current_page_id = self.find_leaf(start)?;

        loop {
            let frame_idx = self.buffer_pool.get_page(current_page_id)?;
            let page = &self.buffer_pool.pool[frame_idx];

            // Parse leaf keys and values from raw page bytes
            let (keys, values, next) = self.parse_leaf_page(page);

            for (i, &k) in keys.iter().enumerate() {
                if k > end {
                    // Done with range scan
                    self.buffer_pool.unpin_page(current_page_id, false);
                    return Ok(results);
                }
                if k >= start {
                    results.push((k, values[i]));
                }
            }

            let next_page = next;
            self.buffer_pool.unpin_page(current_page_id, false);

            if next_page.is_invalid() {
                break;
            }
            current_page_id = next_page;
        }

        Ok(results)
    }

    /// Traverse from root to the leaf that should contain the given key.
    fn find_leaf(&mut self, key: i64) -> Result<PageId, BPlusTreeError> {
        let mut current = self.root_page_id;

        loop {
            let frame_idx = self.buffer_pool.get_page(current)?;
            let page = &self.buffer_pool.pool[frame_idx];
            let page_type = PageType::from_u8(page.data[0]);

            match page_type {
                PageType::Leaf => {
                    self.buffer_pool.unpin_page(current, false);
                    return Ok(current);
                }
                PageType::Internal => {
                    let (keys, children) = self.parse_internal_page(page);

                    // Find the child to descend into
                    let mut child_idx = 0;
                    for (i, &k) in keys.iter().enumerate() {
                        if key >= k {
                            child_idx = i + 1;
                        }
                    }

                    let next = children[child_idx];
                    self.buffer_pool.unpin_page(current, false);
                    current = next;
                }
                _ => return Err(BPlusTreeError::TreeCorrupted("invalid page type".into())),
            }
        }
    }

    /// Search within a specific leaf page for a key.
    fn search_in_leaf(
        &mut self,
        page_id: PageId,
        key: i64,
    ) -> Result<Option<RecordId>, BPlusTreeError> {
        let frame_idx = self.buffer_pool.get_page(page_id)?;
        let page = &self.buffer_pool.pool[frame_idx];
        let (keys, values, _next) = self.parse_leaf_page(page);

        // Binary search
        match keys.binary_search(&key) {
            Ok(idx) => {
                let result = Some(values[idx]);
                self.buffer_pool.unpin_page(page_id, false);
                Ok(result)
            }
            Err(_) => {
                self.buffer_pool.unpin_page(page_id, false);
                Ok(None)
            }
        }
    }

    /// Insert a key-value pair into a specific leaf page.
    /// Handles splits if the leaf overflows.
    fn insert_into_leaf(
        &mut self,
        page_id: PageId,
        key: i64,
        value: RecordId,
    ) -> Result<(), BPlusTreeError> {
        let frame_idx = self.buffer_pool.get_page(page_id)?;
        let page = &mut self.buffer_pool.pool[frame_idx];
        let (mut keys, mut values, _next) = self.parse_leaf_page(page);

        // Insert in sorted order
        let pos = keys.binary_search(&key).unwrap_or_else(|i| i);
        keys.insert(pos, key);
        values.insert(pos, value);

        // Check for overflow
        if keys.len() <= self.config.max_leaf_keys {
            // No split needed -- write back to page
            self.write_leaf_page(page, &keys, &values, PageId::invalid());
            page.is_dirty = true;
            self.buffer_pool.unpin_page(page_id, true);
            return Ok(());
        }

        // Leaf overflow -- split
        let mid = keys.len() / 2;
        let right_keys = keys.split_off(mid);
        let right_values = values.split_off(mid);

        // Allocate new right page
        let right_page_id = self.buffer_pool.allocate_page();
        let right_frame_idx = self.buffer_pool.get_page(right_page_id)?;
        let right_frame = &mut self.buffer_pool.pool[right_frame_idx];

        // Write left page (page_id)
        self.write_leaf_page(page, &keys, &values, right_page_id);
        page.is_dirty = true;

        // Write right page
        self.write_leaf_page(right_frame, &right_keys, &right_values, PageId::invalid());
        right_frame.is_dirty = true;

        let separator_key = right_keys[0];

        // Unpin both pages
        self.buffer_pool.unpin_page(page_id, true);
        self.buffer_pool.unpin_page(right_page_id, true);

        // Push separator key up to parent
        self.insert_into_parent(page_id, separator_key, right_page_id)
    }

    /// Delete a key from a specific leaf page.
    fn delete_from_leaf(
        &mut self,
        page_id: PageId,
        key: i64,
    ) -> Result<(), BPlusTreeError> {
        let frame_idx = self.buffer_pool.get_page(page_id)?;
        let page = &mut self.buffer_pool.pool[frame_idx];
        let (mut keys, mut values, next) = self.parse_leaf_page(page);

        // Find and remove the key
        if let Ok(pos) = keys.binary_search(&key) {
            keys.remove(pos);
            values.remove(pos);
        } else {
            self.buffer_pool.unpin_page(page_id, false);
            return Err(BPlusTreeError::KeyNotFound(key));
        }

        // Check for underflow (only if this is not the root)
        if page_id == self.root_page_id || keys.len() >= self.config.min_keys {
            self.write_leaf_page(page, &keys, &values, next);
            page.is_dirty = true;
            self.buffer_pool.unpin_page(page_id, true);
            return Ok(());
        }

        // Underflow -- handle borrowing or merging
        // (simplified: just write back for now)
        self.write_leaf_page(page, &keys, &values, next);
        page.is_dirty = true;
        self.buffer_pool.unpin_page(page_id, true);
        Ok(())
    }

    /// Insert a key and right child into the parent of a split node.
    /// Recursively handles internal node splits.
    fn insert_into_parent(
        &mut self,
        left_page_id: PageId,
        key: i64,
        right_page_id: PageId,
    ) -> Result<(), BPlusTreeError> {
        // Find parent (simplified -- in a real impl you'd track parent
        // during traversal or maintain a parent pointer)
        let parent_id = self.find_parent_of(left_page_id)?;

        if parent_id.is_invalid() {
            // Left was root -- create new root
            let new_root_id = self.buffer_pool.allocate_page();
            let frame_idx = self.buffer_pool.get_page(new_root_id)?;
            let page = &mut self.buffer_pool.pool[frame_idx];

            // Write new root: one key, two children
            page.data[0] = PageType::Internal as u8;
            // Serialize: [type][num_keys=1][child0][key0][child1]
            let header = PageHeader {
                page_type: PageType::Internal,
                page_id: new_root_id,
                num_keys: 1,
                lsn: 0,
            };
            self.write_internal_page(page, &[key], &[left_page_id, right_page_id]);
            page.is_dirty = true;

            self.root_page_id = new_root_id;
            self.buffer_pool.unpin_page(new_root_id, true);
            return Ok(());
        }

        // Insert into parent
        let frame_idx = self.buffer_pool.get_page(parent_id)?;
        let page = &mut self.buffer_pool.pool[frame_idx];
        let (mut keys, mut children) = self.parse_internal_page(page);

        // Find position and insert
        let pos = keys.binary_search(&key).unwrap_or_else(|i| i);
        keys.insert(pos, key);
        children.insert(pos + 1, right_page_id);

        // Check for overflow
        if keys.len() <= self.config.max_internal_keys {
            self.write_internal_page(page, &keys, &children);
            page.is_dirty = true;
            self.buffer_pool.unpin_page(parent_id, true);
            return Ok(());
        }

        // Internal overflow -- split
        let mid = keys.len() / 2;
        let push_up_key = keys[mid];
        let right_keys = keys.split_off(mid + 1);
        keys.pop(); // remove push_up_key from left
        let right_children = children.split_off(mid + 1);

        self.write_internal_page(page, &keys, &children);
        page.is_dirty = true;

        let right_page_id = self.buffer_pool.allocate_page();
        let right_frame_idx = self.buffer_pool.get_page(right_page_id)?;
        let right_frame = &mut self.buffer_pool.pool[right_frame_idx];
        self.write_internal_page(right_frame, &right_keys, &right_children);
        right_frame.is_dirty = true;

        self.buffer_pool.unpin_page(parent_id, true);
        self.buffer_pool.unpin_page(right_page_id, true);

        // Recursively insert into grandparent
        self.insert_into_parent(parent_id, push_up_key, right_page_id)
    }

    /// Find the parent page of a given page (simplified).
    /// In a real implementation, you'd either maintain parent pointers
    /// or walk the tree from the root.
    fn find_parent_of(&self, _child: PageId) -> Result<PageId, BPlusTreeError> {
        // Simplified: always returns root for now
        // A real implementation would track this during find_leaf()
        Ok(self.root_page_id)
    }

    // -----------------------------------------------------------------------
    // Page serialization / deserialization helpers
    // -----------------------------------------------------------------------

    fn parse_internal_page(&self, page: &Frame) -> (Vec<i64>, Vec<PageId>) {
        // Simplified parsing -- in reality you'd use proper serialization
        let page_type = PageType::from_u8(page.data[0]);
        assert_eq!(page_type, PageType::Internal);

        let num_keys =
            u16::from_le_bytes([page.data[1], page.data[2]]) as usize;

        let mut keys = Vec::with_capacity(num_keys);
        let mut children = Vec::with_capacity(num_keys + 1);

        let mut offset = 3; // after type + num_keys
        for _ in 0..=num_keys {
            let pid = u64::from_le_bytes([
                page.data[offset],
                page.data[offset + 1],
                page.data[offset + 2],
                page.data[offset + 3],
                page.data[offset + 4],
                page.data[offset + 5],
                page.data[offset + 6],
                page.data[offset + 7],
            ]);
            children.push(PageId(pid));
            offset += 8;

            if children.len() <= num_keys {
                let key = i64::from_le_bytes([
                    page.data[offset],
                    page.data[offset + 1],
                    page.data[offset + 2],
                    page.data[offset + 3],
                    page.data[offset + 4],
                    page.data[offset + 5],
                    page.data[offset + 6],
                    page.data[offset + 7],
                ]);
                keys.push(key);
                offset += 8;
            }
        }

        (keys, children)
    }

    fn write_internal_page(
        &self,
        page: &mut Frame,
        keys: &[i64],
        children: &[PageId],
    ) {
        let num_keys = keys.len() as u16;
        page.data[0] = PageType::Internal as u8;
        page.data[1..3].copy_from_slice(&num_keys.to_le_bytes());

        let mut offset = 3;
        for i in 0..children.len() {
            page.data[offset..offset + 8]
                .copy_from_slice(&children[i].0.to_le_bytes());
            offset += 8;

            if i < keys.len() {
                page.data[offset..offset + 8]
                    .copy_from_slice(&keys[i].to_le_bytes());
                offset += 8;
            }
        }
    }

    fn parse_leaf_page(&self, page: &Frame) -> (Vec<i64>, Vec<RecordId>, PageId) {
        let page_type = PageType::from_u8(page.data[0]);
        assert_eq!(page_type, PageType::Leaf);

        let num_keys =
            u16::from_le_bytes([page.data[1], page.data[2]]) as usize;

        let mut offset = 3;
        let mut keys = Vec::with_capacity(num_keys);
        let mut values = Vec::with_capacity(num_keys);

        for _ in 0..num_keys {
            let key = i64::from_le_bytes([
                page.data[offset],
                page.data[offset + 1],
                page.data[offset + 2],
                page.data[offset + 3],
                page.data[offset + 4],
                page.data[offset + 5],
                page.data[offset + 6],
                page.data[offset + 7],
            ]);
            keys.push(key);
            offset += 8;

            let rid_page = u64::from_le_bytes([
                page.data[offset],
                page.data[offset + 1],
                page.data[offset + 2],
                page.data[offset + 3],
                page.data[offset + 4],
                page.data[offset + 5],
                page.data[offset + 6],
                page.data[offset + 7],
            ]);
            let rid_slot = u16::from_le_bytes([
                page.data[offset + 8],
                page.data[offset + 9],
            ]);
            values.push(RecordId {
                page_id: PageId(rid_page),
                slot: rid_slot,
            });
            offset += 10;
        }

        // Next leaf pointer (8 bytes after last record)
        let next_pid = u64::from_le_bytes([
            page.data[offset],
            page.data[offset + 1],
            page.data[offset + 2],
            page.data[offset + 3],
            page.data[offset + 4],
            page.data[offset + 5],
            page.data[offset + 6],
            page.data[offset + 7],
        ]);
        let next = PageId(next_pid);

        (keys, values, next)
    }

    fn write_leaf_page(
        &self,
        page: &mut Frame,
        keys: &[i64],
        values: &[RecordId],
        next: PageId,
    ) {
        let num_keys = keys.len() as u16;
        page.data[0] = PageType::Leaf as u8;
        page.data[1..3].copy_from_slice(&num_keys.to_le_bytes());

        let mut offset = 3;
        for (key, val) in keys.iter().zip(values.iter()) {
            page.data[offset..offset + 8].copy_from_slice(&key.to_le_bytes());
            offset += 8;
            page.data[offset..offset + 8]
                .copy_from_slice(&val.page_id.0.to_le_bytes());
            page.data[offset + 8..offset + 10]
                .copy_from_slice(&val.slot.to_le_bytes());
            offset += 10;
        }

        // Write next pointer
        page.data[offset..offset + 8].copy_from_slice(&next.0.to_le_bytes());
    }
}

impl PageType {
    fn from_u8(val: u8) -> Self {
        match val {
            1 => PageType::Internal,
            2 => PageType::Leaf,
            _ => PageType::Invalid,
        }
    }
}
```

### Usage Example

```rust
fn main() -> Result<(), BPlusTreeError> {
    let config = BPlusTreeConfig::default_8kb();
    let bpm = BufferPoolManager::new(64, PathBuf::from("mydb.dat"));
    let mut tree = BPlusTree::new(bpm, config);

    // Insert some records
    for i in 0..1000 {
        let key = i * 7 % 997;  // some pseudo-random ordering
        tree.insert(
            key,
            RecordId {
                page_id: PageId(50 + (i as u64 % 10)),
                slot: (i % 100) as u16,
            },
        )?;
    }

    // Point lookup
    if let Some(record_id) = tree.search(42)? {
        println!("Found key 42 -> {:?}", record_id);
    }

    // Range scan
    let results = tree.range_scan(100, 200)?;
    println!("Keys in range [100, 200]: {}", results.len());

    // Flush all dirty pages to disk
    tree.buffer_pool.flush_all()?;

    Ok(())
}
```

---

## 13. Concurrency Considerations

### Latches vs Locks

```
+-----------+---------------------------+----------------------------+
| Concept   | Lock                      | Latch                      |
+-----------+---------------------------+----------------------------+
| Purpose   | Protect data from         | Protect in-memory          |
|           | concurrent transactions   | structures from concurrent |
|           | (logical consistency)     | threads (physical safety)  |
+-----------+---------------------------+----------------------------+
| Duration  | Transaction lifetime      | Operation duration         |
|           | (could be seconds/minutes)| (microseconds)             |
+-----------+---------------------------+----------------------------+
| Managed by| Transaction manager       | Buffer pool / B-tree code  |
+-----------+---------------------------+----------------------------+
| Granularity| Row, table, database     | Page, node                |
+-----------+---------------------------+----------------------------+
| Example   | SELECT ... FOR UPDATE     | read_lock(page) before     |
|           |                           | reading keys in a node     |
+-----------+---------------------------+----------------------------+
```

**Latches** are lightweight synchronization primitives (like mutexes or RWLocks in Rust) that protect a B+ tree node while it is being accessed or modified.

### Latch Crabbing (Coupling) Protocol

The fundamental protocol for safe concurrent B+ tree traversal. The idea: hold a latch on the current node, acquire the latch on the child, then release the parent latch if the child is "safe."

```
SEARCH (read-only traversal):

    1. Acquire read latch on root
    2. Read root, determine child
    3. Acquire read latch on child
    4. Release parent read latch
    5. Repeat until leaf

    At any point, you hold at most 2 latches (parent + child).
```

```
INSERT (may need to modify nodes):

    1. Acquire write latch on root
    2. Determine if child is "safe" (won't split)
       a. If child is SAFE:
          - Acquire write latch on child
          - Release ALL ancestor latches
          - Continue down with only child latch held
       b. If child is NOT safe:
          - Acquire write latch on child
          - Keep parent latch held (may need to split parent)
    3. Repeat until leaf
    4. Modify leaf, split if needed (you hold latches to all
       ancestors that might need modification)

    "Safe" = node has room and won't need to split.
    "Unsafe" = node is full and will split when modified.
```

### Visual: Latch Crabbing for Insert

```
Inserting key 57 into tree:

Step 1: Write-latch root (Page 1)
        Page 1: [50, 100] -> Page 3 is child
        Page 3 is "unsafe" (full) -> keep Page 1 latch
        
Step 2: Write-latch Page 3
        Page 3: [65, 80] -> Page 8 is child
        Page 8 is "safe" (has room) -> release Page 1 latch!
        
        NOW: only holding latch on Page 3
        
Step 3: Write-latch Page 8
        Release Page 3 latch
        
        NOW: only holding latch on Page 8
        
Step 4: Insert 57 into Page 8
        Page 8 splits -> need to modify parent (Page 3)
        But we released Page 3 latch!
        
        -> We need to re-acquire latches bottom-up.
```

### Corrected Latch Crabbing for Insert (with split handling)

```
The actual protocol used in practice (e.g., BusTub at CMU):

1. Top-down: acquire latches using crabbing protocol
2. If a node is safe at any point, release all ancestor latches
3. If you reach the leaf without finding a safe node, you hold
   latches all the way down
4. After the operation, release latches bottom-up

This guarantees:
- You can always complete the operation
- You hold at most O(height) latches in the worst case
- In the common case (non-splitting insert), you hold O(1) latches
```

### Lock Manager vs Latch Manager

```
Lock Manager (for transactions):
    - table_lock(table_id, EXCLUSIVE)
    - row_lock(record_id, SHARED)
    - deadlock detection (wait-for graph)
    - grant/wait semantics

Latch Manager (for B-tree internals):
    - page_latch(page_id, READ/WRITE)
    - no deadlock possible (consistent ordering)
    - immediate acquire or block
    - implemented with std::sync::RwLock or parking_lot
```

### Rust Concurrency Sketch

```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

struct ConcurrentBPlusTree {
    root: Arc<RwLock<PageId>>,
    latch_table: HashMap<PageId, Arc<RwLock<()>>>,
}

impl ConcurrentBPlusTree {
    /// Acquire a read latch on a page.
    /// Blocks if another thread holds a write latch.
    fn acquire_read_latch(&self, page_id: PageId) -> std::sync::RwLockReadGuard<()> {
        // In reality, the latch is stored in the buffer pool frame.
        // This is a simplified representation.
        unimplemented!()
    }

    /// Acquire a write latch on a page.
    /// Blocks if another thread holds any latch.
    fn acquire_write_latch(&self, page_id: PageId) -> std::sync::RwLockWriteGuard<()> {
        unimplemented!()
    }

    /// Safe insert with latch crabbing.
    fn safe_insert(&self, key: i64, value: RecordId) -> Result<(), BPlusTreeError> {
        let mut ancestors: Vec<PageId> = Vec::new();
        let mut current = *self.root.read().unwrap();

        loop {
            // Acquire write latch on current node
            let _guard = self.acquire_write_latch(current);

            // Check if this node is safe for modification
            let is_safe = self.node_has_room(current);

            if is_safe {
                // Safe! Release all ancestor latches
                ancestors.clear();
            }

            // Descend to child (for internal nodes)
            let page_type = self.get_page_type(current);
            if page_type == PageType::Internal {
                let child = self.find_child_for_key(current, key);
                ancestors.push(current);
                current = child;
            } else {
                // We're at a leaf
                self.insert_into_leaf_unsafe(current, key, value)?;
                break;
            }
        }

        Ok(())
    }

    fn node_has_room(&self, _page_id: PageId) -> bool {
        // Check if the node will need to split
        unimplemented!()
    }

    fn get_page_type(&self, _page_id: PageId) -> PageType {
        unimplemented!()
    }

    fn find_child_for_key(&self, _page_id: PageId, _key: i64) -> PageId {
        unimplemented!()
    }

    fn insert_into_leaf_unsafe(
        &mut self,
        _page_id: PageId,
        _key: i64,
        _value: RecordId,
    ) -> Result<(), BPlusTreeError> {
        unimplemented!()
    }
}
```

---

## 14. WAL Integration with B-Tree Operations

### What Is a WAL?

A Write-Ahead Log ensures durability: if the database crashes, committed transactions can be recovered. The fundamental rule:

```
WAL RULE:
    Before writing any page to disk, write a log record to the WAL file
    describing the change. The log record must be flushed (fsync'd) before
    the modified page is written to disk.

    WAL record flushed to disk  BEFORE  page flushed to disk
    
    On crash recovery:
    1. Read WAL from beginning
    2. Redo committed transactions
    3. Undo uncommitted transactions
    4. Database is consistent
```

### Why WAL + B-Tree Is Tricky

B-tree operations are not atomic. A single insert might modify multiple pages:

```
Insert that causes a split:

    1. Modify leaf page (split)
    2. Modify parent page (add separator key)
    3. If root splits: modify/create root page

    Three pages modified. Crash between steps 1 and 2 = tree is corrupt.
```

### Solution: physiological logging

Log at the **page level**, not the logical level. Each log record describes a modification to a single page.

```
Log record format:
+----------+----------+---------+-----------+
| LSN      | PageID   | Offset  | Length    |
| (8 bytes)| (8 bytes)| (4 bytes)| (4 bytes)|
+----------+----------+---------+-----------+
| Data (before image or after image)        |
+-------------------------------------------+
```

### WAL Record Types for B-Tree Operations

```
+--------------------+-------------------------------------------+
| Operation          | WAL Records                               |
+--------------------+-------------------------------------------+
| Insert (no split)  | 1. UpdateLeaf(page=8, key=57, val=...)    |
|                    | Total: 1 log record                       |
+--------------------+-------------------------------------------+
| Insert (with split)| 1. SplitLeaf(page=8, new_page=14)        |
|                    | 2. UpdateLeaf(page=8, ...)                |
|                    | 3. UpdateLeaf(page=14, ...)               |
|                    | 4. UpdateInternal(parent=3, ...)          |
|                    | Total: 4 log records                      |
+--------------------+-------------------------------------------+
| Delete             | 1. UpdateLeaf(page=5, ...)                |
|                    | 2. If merge: UpdateInternal(parent, ...)  |
|                    | Total: 1-2 log records                    |
+--------------------+-------------------------------------------+
```

### LSN (Log Sequence Number)

Every page stores the LSN of the last log record that modified it. This is used during recovery to determine which log records need to be replayed.

```
Page header contains:
    lsn: u64    // LSN of the last modification to this page

WAL contains:
    records ordered by LSN
    
Recovery:
    for each log record (in LSN order):
        if record.lsn > page.lsn:
            // This log record has not been applied to the page yet
            // Redo the operation on the page
```

### Checkpointing

Periodically, the database writes a checkpoint to the WAL:

```
Checkpoint record contains:
    - List of all dirty page IDs and their current LSNs
    - List of all active transactions

Recovery after crash:
    1. Find the last checkpoint in the WAL
    2. Start from the checkpoint
    3. For each subsequent log record:
       - If transaction was committed: REDO
       - If transaction was not committed: UNDO
```

### Integration with Buffer Pool

```rust
/// When modifying a page through the buffer pool, the WAL must be
/// updated before the dirty page is flushed.
struct WALPageGuard<'a> {
    bpm: &'a mut BufferPoolManager,
    wal: &'a mut WriteAheadLog,
    page_id: PageId,
    frame_idx: usize,
}

impl<'a> WALPageGuard<'a> {
    /// Write a log record and return an LSN.
    /// The LSN must be written to the page header before the page
    /// is unpinned.
    fn log_and_modify(
        &mut self,
        log_record: &LogRecord,
    ) -> Result<u64, BPlusTreeError> {
        // Step 1: Append log record to WAL
        let lsn = self.wal.append_record(log_record)?;

        // Step 2: Flush WAL to disk (fsync)
        self.wal.flush()?;

        // Step 3: Modify the page data
        // (caller does the actual modification)

        // Step 4: Update page header with new LSN
        let page = &mut self.bpm.pool[self.frame_idx];
        page.data[5..13].copy_from_slice(&lsn.to_le_bytes());
        page.is_dirty = true;

        Ok(lsn)
    }
}

impl<'a> Drop for WALPageGuard<'a> {
    fn drop(&mut self) {
        // Unpin the page (marks it dirty)
        self.bpm.unpin_page(self.page_id, true);
    }
}
```

### Recovery Algorithm (ARIES-style)

```
Analysis Phase:
    - Scan WAL from last checkpoint
    - Build dirty page table and transaction table
    - Determine which pages were dirty at crash time

Redo Phase:
    - Replay ALL log records (committed or not)
    - For each record: if page_lsn < record_lsn, apply the change
    - This brings the database to the exact state at crash time

Undo Phase:
    - For each uncommitted transaction, walk the log backward
    - Undo each change in reverse order
    - Write compensation log records (CLRs) for each undo
```

---

## 15. Practical Tips for Building Your Own Database

### Start Simple, Then Iterate

```
Phase 1: Get a working B+ tree
    - Single-threaded
    - No WAL
    - No concurrency
    - Simple file I/O (pread/pwrite)
    - Focus: get splits and merges correct

Phase 2: Add a buffer pool
    - LRU eviction
    - Pin/unpin protocol
    - Dirty page flushing
    - Focus: performance, cache hit ratio

Phase 3: Add WAL
    - Simple append-only log
    - Recovery on restart
    - Focus: crash safety

Phase 4: Add concurrency
    - Latch crabbing
    - Multiple reader threads
    - Single writer thread (start here)
    - Focus: correctness under concurrent access

Phase 5: Add transactions
    - MVCC or 2PL
    - Deadlock detection
    - Focus: isolation levels
```

### Common Pitfalls

```
1. Off-by-one errors in page splits
   - The separator key logic is tricky
   - Leaf split: copy key to parent
   - Internal split: push key to parent (remove from node)
   - Get this wrong and you lose data or corrupt the tree

2. Forgetting to update leaf linked list
   - After a split, the new leaf must be linked
   - After a merge, the old leaf must be unlinked
   - Missing this breaks range scans

3. Not handling duplicate keys
   - Decide: store duplicates in leaf (append to value list)
   - Or: use (key, primary_key) as the comparison key
   - Postgres handles this well; study their approach

4. Page alignment issues
   - Every page must be exactly PAGE_SIZE bytes
   - Internal fragmentation wastes space
   - Use a proper slot array, not just append

5. Not flushing WAL before pages
   - WAL rule violation = data loss on crash
   - Use fsync(), not just flush()
   - On Linux, O_DIRECT bypasses page cache (good for large DBs)

6. Not testing with random operations
   - Sequential insert/delete is easy
   - Random operations find edge cases in splits/merges
   - Use a property-based testing framework (proptest in Rust)
```

### Useful References

```
Books:
- "Database Internals" by Alex Petrov (best overview of storage engines)
- "Designing Data-Intensive Applications" by Martin Kleppmann
- "Architecture of a Database System" (paper by Hellerstein et al.)

Code to study:
- CMU BusTub: https://github.com/cmu-db/bustub
  (Educational B+ tree + buffer pool, great for learning)
- SQLite: https://sqlite.org/src
  (Production-quality B+ tree, surprisingly readable C code)
- LMDB: https://github.com/LMDB/lmdb
  (Minimal, elegant B+ tree implementation)
- CockroachDB: Pebble storage engine
  (Go implementation of LSM, but good design reference)
- Rust-specific:
  - https://github.com/skyzh/mini-lsm (LSM, but shows Rust patterns)
  - https://github.com/jonhoo/rust-merkle-tree (Merkle tree in Rust)

Key papers:
- "The Log-Structured Merge-Tree (LSM-Tree)" (O'Neil et al.)
- "ARIES: A Transaction Recovery Method" (Mohan et al.)
- "The R-Tree: A Dynamic Index" (but shows page-based index ideas)
- "Bw-Tree: A B-tree for New Hardware Platforms" (Intel)
```

### Rust-Specific Advice

```
1. Use the bytes crate for page buffers
   - Avoids manual byte manipulation
   - Zero-copy slicing is very useful for parsing page contents

2. Use mmap with caution
   - It seems easy but introduces subtle bugs
   - Page faults = implicit I/O = unpredictable latency
   - Explicit pread/pwrite is more predictable

3. Use parking_lot over std sync primitives
   - parking_lot::RwLock is faster and has better API
   - parking_lot::Mutex has poisoning disabled by default

4. Consider a slab allocator for frames
   - Pre-allocate all frame buffers upfront
   - Avoids heap fragmentation from many small allocations

5. Profile early
   - Use perf (Linux) or Instruments (macOS) to find hot paths
   - Buffer pool hit ratio is your most important metric
   - 99%+ hit ratio is typical for well-tuned systems

6. Test with Miri
   - cargo +nightly miri test
   - Catches undefined behavior, data races, memory bugs
   - Essential for unsafe code in buffer pool management
```

### A Note on Real-World Complexity

```
What this guide covers:
    - Core B+ tree data structure
    - Page-based storage
    - Basic buffer pool
    - WAL fundamentals
    - Concurrency basics

What a production database additionally needs:
    - Compression (page-level: snappy, zstd, lz4)
    - Encryption (page-level AES)
    - Backup/restore (incremental, page-level)
    - Replication (WAL shipping, logical replication)
    - Vacuum/compaction (reclaiming deleted space)
    - Statistics (for query planner)
    - Multi-version concurrency control (MVCC)
    - Savepoints and nested transactions
    - Two-phase locking (2PL) or MVCC for isolation
    - Deadlock detection and resolution
    - Query execution engine
    - Network protocol (PostgreSQL wire protocol, etc.)
    
A database is one of the most complex software systems.
Take it one step at a time, and test everything.
```

---

*This document is a living reference. As you build your database, come back to update and expand the sections that are most relevant to your current work.*

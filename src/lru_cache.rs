use std::{collections::HashMap, hash::Hash};

pub struct Node<K, V> {
    key: K,
    value: V,
    next: Option<usize>,
    prev: Option<usize>,
}

pub struct LRUCache<K, V> {
    nodes: Vec<Option<Node<K, V>>>,
    map: HashMap<K, usize>,
    head: Option<usize>,
    tail: Option<usize>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::new(),
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let &idx = self.map.get(key)?;
        self.detach(idx);
        self.attach_head(idx);
        Some(&self.nodes[idx].as_ref()?.value)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return;
        }

        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].as_mut().unwrap().value = value;
            self.detach(idx);
            self.attach_head(idx);
            return;
        }

        if self.map.len() >= self.capacity {
            let tail_idx = self.tail.unwrap();
            let evict_key = self.nodes[tail_idx].as_ref().unwrap().key.clone();
            self.map.remove(&evict_key);

            self.detach(tail_idx);

            let key_clone = key.clone();

            self.nodes[tail_idx] = Some(Node {
                key,
                value,
                next: None,
                prev: None,
            });

            self.map.insert(key_clone, tail_idx);

            self.attach_head(tail_idx);

            return;
        }

        let idx = self.nodes.len();

        self.nodes.push(Some(Node {
            key: key.clone(),
            value,
            next: None,
            prev: None,
        }));

        self.map.insert(key, idx);

        self.attach_head(idx);
    }

    fn attach_head(&mut self, idx: usize) {
        if idx >= self.capacity {
            println!("Out of bound index value");
            return;
        }

        let node = self.nodes[idx].as_mut().unwrap();
        node.prev = None;
        node.next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].as_mut().unwrap().prev = Some(idx);
        } else {
            self.tail = Some(idx);
        }

        self.head = Some(idx);
    }

    fn detach(&mut self, idx: usize) {
        if idx >= self.capacity {
            println!("Out of bound index value");
        }

        let node = self.nodes[idx].as_mut().unwrap();
        let prev = node.prev;
        let next = node.next;

        if let Some(prev_idx) = prev {
            self.nodes[prev_idx].as_mut().unwrap().next = next;
        } else {
            self.head = next;
        }

        if let Some(next_idx) = next {
            self.nodes[next_idx].as_mut().unwrap().prev = prev;
        } else {
            self.tail = prev;
        }
    }
}

pub struct SIEVE<T> {
    queue: Vec<(T, bool)>,
    hand: usize,
    capacity: usize,
}

impl<T> SIEVE<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Vec::with_capacity(capacity),
            hand: 0,
            capacity,
        }
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() == self.capacity
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn hand_pos(&self) -> usize {
        self.hand
    }

    pub fn ref_bit(&self, idx: usize) -> Option<bool> {
        self.queue.get(idx).map(|(_, visited)| *visited)
    }

    pub fn add(&mut self, value: T) -> usize {
        if !self.is_full() {
            self.queue.push((value, false));
        } else {
            self.remove();
            self.queue.push((value, false));
        }
        self.queue.len() - 1
    }

    pub fn get(&mut self, idx: usize) -> Option<&T> {
        self.queue.get_mut(idx).map(|(val, visited)| {
            *visited = true;
            &*val
        })
    }

    pub fn remove(&mut self) -> usize {
        loop {
            if self.queue[self.hand].1 {
                self.queue[self.hand].1 = false;
                self.hand = (self.hand + 1) % self.capacity;
            } else {
                let victim = self.hand;
                self.queue.remove(victim);
                // remove shifts elements left — hand now points to the next item.
                // Only wrap if victim was the last element (hand fell off the end).
                if self.hand >= self.queue.len() {
                    self.hand = 0;
                }
                return victim;
            }
        }
    }
}

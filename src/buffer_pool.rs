use std::collections::VecDeque;
use std::sync::Mutex;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub enum PoolError {
    InvalidIndex(usize),
    NotAcquired(usize),
    AlreadyReleased(usize),
}

type Buffer = Box<[u8; PAGE_SIZE]>;

struct BufferPoolInner {
    buffers: Vec<Option<Buffer>>,
    free_list: VecDeque<usize>,
}

pub struct BufferPool {
    inner: Mutex<BufferPoolInner>,
}

impl BufferPool {
    pub fn new(num_pages: usize) -> Self {
        let mut buffers = Vec::with_capacity(num_pages);

        for _ in 0..num_pages {
            buffers.push(Some(Box::new([0u8; PAGE_SIZE])));
        }

        let free_list = (0..num_pages).collect();

        Self {
            inner: Mutex::new(BufferPoolInner { buffers, free_list }),
        }
    }

    pub fn capacity(&self) -> usize {
        self.inner.lock().unwrap().buffers.len()
    }

    pub fn available(&self) -> usize {
        self.inner.lock().unwrap().free_list.len()
    }

    pub fn acquire(&self) -> Option<(Buffer, usize)> {
        let mut inner = self.inner.lock().unwrap();

        let idx = inner.free_list.pop_front()?;

        let buf = inner.buffers[idx].take()?;

        Some((buf, idx))
    }

    pub fn release(&self, idx: usize, mut buf: Buffer) -> Result<(), PoolError> {
        let mut inner = self.inner.lock().unwrap();

        if idx >= inner.buffers.len() {
            return Err(PoolError::InvalidIndex(idx));
        }

        if inner.buffers[idx].is_some() {
            return Err(PoolError::AlreadyReleased(idx));
        }

        for b in buf.iter_mut() {
            *b = 0;
        }

        inner.buffers[idx] = Some(buf);
        inner.free_list.push_back(idx);

        Ok(())
    }
}

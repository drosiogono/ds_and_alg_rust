use std::alloc::{Layout, alloc, dealloc};

#[derive(Debug)]
pub enum QueueError {
    CapacityOverflow,
    InvalidCapacity,
    Underflow,
    LayoutError(std::alloc::LayoutError),
}

impl std::fmt::Display for QueueError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::CapacityOverflow => write!(f, "Capacity overflow"),
            Self::InvalidCapacity => write!(f, "Invalid capacity"),
            Self::Underflow => write!(f, "Underflow"),
            Self::LayoutError(layout_error) => write!(f, "LayoutError: {}", layout_error),
        }
    }
}

impl From<std::alloc::LayoutError> for QueueError {
    fn from(layout_error: std::alloc::LayoutError) -> QueueError {
        QueueError::LayoutError(layout_error)
    }
}

impl std::error::Error for QueueError {}

#[derive(Debug)]
pub struct IntQueue {
    ptr: *mut i32,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl Drop for IntQueue {
    fn drop(&mut self) {
        let layout = Layout::array::<i32>(self.capacity).unwrap();
        unsafe {
            dealloc(
                self.ptr as *mut u8,
                layout,
            )
        };
    }
}

impl IntQueue {
    pub fn new(capacity: usize) -> Result<IntQueue, QueueError> {
        if capacity < 2 {
            Err(QueueError::InvalidCapacity)
        } else {
            let layout = Layout::array::<i32>(capacity)?;
            let ptr = unsafe {
                alloc(layout) as *mut i32
            };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            Ok(IntQueue {
                ptr,
                head: 0,
                tail: 0,
                capacity,
            })
        }
    }

    fn grow(&mut self) -> Result<(), QueueError> {
        let new_capacity = self.capacity
            .checked_mul(2)
            .ok_or(QueueError::CapacityOverflow)?;
        let old = Layout::array::<i32>(self.capacity)?;
        let new = Layout::array::<i32>(new_capacity)?;
        
        let new_ptr = unsafe {
            alloc(new) as *mut i32
        };
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new);
        }
        let n = self.size();
        for i in 0..n {
            unsafe {
                new_ptr.add(i + 1).write(
                    self.ptr.add((self.tail + i + 1) % self.capacity).read()
                )
            };
        }
        unsafe {
            dealloc(
                self.ptr as *mut u8,
                old,
            )
        }

        self.ptr = new_ptr;
        self.head = n;
        self.tail = 0;
        self.capacity = new_capacity;
        Ok(())
    }

    pub fn enqueue(&mut self, value: i32) -> Result<(), QueueError> {
        if self.is_full() {
            self.grow()?;
        }
        self.head = (self.head + 1) % self.capacity;
        unsafe {
            self.ptr.add(self.head).write(value)
        };
        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<i32, QueueError> {
        if self.is_empty() {
            Err(QueueError::Underflow)
        } else {
            self.tail = (self.tail + 1) % self.capacity;
            let value = unsafe {
                self.ptr.add(self.tail).read()
            };
            Ok(value)
        }
    }

    pub fn peek(&self) -> Result<i32, QueueError> {
        if self.is_empty() {
            Err(QueueError::Underflow)
        } else {
            let value = unsafe {
                self.ptr.add((self.tail + 1) % self.capacity).read()
            };
            Ok(value)
        }
    }

    pub fn is_full(&self) -> bool {
        (self.head + 1) % self.capacity == self.tail
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn size(&self) -> usize {
        (self.head - self.tail + self.capacity) % self.capacity
    }
}
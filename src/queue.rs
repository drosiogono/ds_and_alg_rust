use std::alloc::{Layout, alloc, dealloc};
use std::ptr;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum QueueError {
    CapacityOverflow,
    // InvalidCapacity,
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
            // Self::InvalidCapacity => write!(f, "Invalid capacity"),
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
pub struct Queue<T> {
    ptr: *mut T,
    head: usize,
    tail: usize,
    capacity: usize,
    phantom: PhantomData<T>,
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.size() {
                ptr::drop_in_place(
                    self.ptr
                        .add((self.tail + i + 1) % self.capacity)
                );
            } // Assumption: drops of T in drop() do not panic.
            if !self.ptr.is_null() {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                dealloc(
                    self.ptr as *mut u8,
                    layout,
                )
            }
        };
    }
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        if std::mem::size_of::<T>() == 0 {
            panic!("Queue does not support zero-sized types.");
        }
        Self {
            ptr: ptr::null_mut(),
            head: 0,
            tail: 0,
            capacity: 1,
            phantom: PhantomData,
        }
    }

    fn grow(&mut self) -> Result<(), QueueError> {
        let new_capacity = if self.capacity == 1 {
            4
        } else { self.capacity
            .checked_mul(2)
            .ok_or(QueueError::CapacityOverflow)?
        };
        let old = Layout::array::<T>(self.capacity)?;
        let new = Layout::array::<T>(new_capacity)?;
        
        let new_ptr = unsafe {
            alloc(new) as *mut T
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
        if !self.ptr.is_null() {
            unsafe {
                dealloc(
                    self.ptr as *mut u8,
                    old,
                )
            }
        }

        self.ptr = new_ptr;
        self.head = n;
        self.tail = 0;
        self.capacity = new_capacity;
        Ok(())
    }

    pub fn enqueue(&mut self, value: T) -> Result<(), QueueError> {
        if self.is_full() {
            self.grow()?;
        }
        self.head = (self.head + 1) % self.capacity;
        unsafe {
            self.ptr.add(self.head).write(value)
        };
        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<T, QueueError> {
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

    pub fn peek(&self) -> Result<&T, QueueError> {
        if self.is_empty() {
            Err(QueueError::Underflow)
        } else {
            let value = unsafe {
                &*self.ptr.add((self.tail + 1) % self.capacity)
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
        (self.head + self.capacity - self.tail) % self.capacity
    }
}

impl<T, const N: usize> From<[T; N]> for Queue<T> {
    fn from(s: [T; N]) -> Self {
        if std::mem::size_of::<T>() == 0 {
            panic!("Queue does not support zero-sized types.");
        }
        if N == 0 {
            Self::new()
        } else {
            let layout = Layout::array::<T>(N + 1).expect("Layout error");
            let ptr = unsafe {
                alloc(layout) as *mut T
            };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            for (i, v) in s.into_iter().enumerate() {
                unsafe {
                    ptr.add(i + 1).write(v);
                };
            }
            Self {
                ptr,
                head: N,
                tail: 0,
                capacity: N + 1,
                phantom: PhantomData,
            }
        }
    }
}

impl<T> From<Vec<T>> for Queue<T> {
    fn from(s: Vec<T>) -> Self {
        if std::mem::size_of::<T>() == 0 {
            panic!("Queue does not support zero-sized types.");
        }
        let n = s.len();
        if n == 0 {
            Self::new()
        } else {
            let layout = Layout::array::<T>(n + 1).expect("Layout error");
            let ptr = unsafe {
                alloc(layout) as *mut T
            };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            for (i, v) in s.into_iter().enumerate() {
                unsafe {
                    ptr.add(i + 1).write(v);
                };
            }
            Self {
                ptr,
                head: n,
                tail: 0,
                capacity: n + 1,
                phantom: PhantomData,
            }
        }
    }
}
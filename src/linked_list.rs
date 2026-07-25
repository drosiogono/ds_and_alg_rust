use std::alloc::{alloc, dealloc, Layout};
// use std::mem::MaybeUninit;
use std::ptr;

pub enum LinkedListError {
    IndexError(usize),
}

impl std::fmt::Display for LinkedListError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::IndexError(idx) => write!(f, "invalid index: {idx}"),
        }
    }
}

// Hmm I want to practice MaybeUninit<T>.. 
// But it's not suitable because it's not sure whether it implements Copy trait
#[derive(Clone, Copy)]
struct Node<T> {
    now: *mut T,
    next: *mut Node<T>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        if !self.now.is_null() {
            unsafe {
                self.now.drop_in_place()
            };
        }
        let layout = Layout::new::<T>();
        unsafe {
            dealloc(self.now as *mut u8, layout)
        };
    }
}

impl<T> Node<T> {
    fn new(maybe_value: Option<T>) -> Self {
        match maybe_value {
            Some(value) => {
                let layout = Layout::new::<T>();
                let ptr = unsafe {
                    alloc(layout) as *mut T
                };
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                unsafe {
                    ptr.write(value)
                };
                Self {
                    now: ptr,
                    next: ptr::null_mut(),
                }
            },
            None => Self {
                now: ptr::null_mut(),
                next: ptr::null_mut(),
            }
        }
    }
    fn link(&mut self, tail: &mut Node<T>) {
        self.next = tail;
    }
    fn next(&self) -> Option<*mut Node<T>> {
        if self.next.is_null() {
            None
        } else {
            Some(self.next)
        }
    }
}

pub struct LinkedList<T> {
    ptr: Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            ptr: Node::new(None),
            len: 0,
        }
    }
    pub fn insert(&mut self, idx: usize, value: T) -> Result<(), LinkedListError> {
        if idx > self.len {
            Err(LinkedListError::IndexError(idx))
        } else {
            let mut ptr = self.ptr;
            for _ in 0..idx {
                unsafe {
                    ptr = *ptr.next().ok_or(LinkedListError::IndexError(idx))?;
                }
            }
            Ok(())
        }
    }
}
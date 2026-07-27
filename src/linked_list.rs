use std::alloc::{alloc, dealloc, Layout};
// use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};

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

// Hmm I wanted to practice MaybeUninit<T>.. 
// BIG FIX: changed Node.now from *mut T to T
struct Node<T> {
    now: T,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            now: value,
            next: ptr::null_mut(),
        }
    }
    fn next(&self) -> Option<NonNull<Node<T>>> {
        NonNull::new(self.next)
    }
}

pub struct LinkedList<T> {
    ptr: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
        }
    }
    pub fn insert(&mut self, idx: usize, value: T) -> Result<(), LinkedListError> {
        if idx > self.len {
            Err(LinkedListError::IndexError(idx))
        } else {
            let mut node = self.ptr;
            let mut new_node = Box::new(Node::new(value));
            if idx > 0 {
                for _ in 0..(idx - 1) {
                    unsafe {
                        node = (*node).next
                    };
                    if node.next.is_null() {
                        return Err(LinkedListError::IndexError(idx))
                    }
                }
                unsafe {
                    let next = (*node).next().ok_or(LinkedListError::IndexError(idx))?;
                    new_node.next = next;
                    (*node).next = &mut new_node as *mut Node<T>;
                }
            } else {
                new_node.next = node;
                self.ptr = &mut new_node as *mut Node<T>;
            }
            Ok(())
        }
    }
}
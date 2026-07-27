use std::alloc::{alloc, dealloc, Layout};
// use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};
use std::usize;

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
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> NonNull<Node<T>> {
        let b = Box::new(Self {
            now: value,
            next: None,
        });
        NonNull::new(Box::into_raw(b)).expect("Error while allocating a new node.")
    }
    unsafe fn destroy(ptr: NonNull<Node<T>>) {
        drop(Box::from_raw(ptr.as_ptr()));
    }
}

pub struct LinkedList<T> {
    ptr: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            ptr: None,
            len: 0,
        }
    }
    pub fn insert(&mut self, idx: usize, value: T) -> Result<(), LinkedListError> {
        if idx > self.len {
            Err(LinkedListError::IndexError(idx))
        } else {
            let node = self.ptr;
            if idx == 0 {
                let new_node = Node::new(value).as_ptr();
                unsafe {
                    (*new_node).next = node
                };
                self.ptr = NonNull::new(new_node);
            } else {
                let mut prev = node.ok_or(LinkedListError::IndexError(idx))?;
                for _ in 0..(idx - 1) {
                    unsafe {
                        prev = (*prev.as_ptr()).next.ok_or(LinkedListError::IndexError(idx))?
                    };
                }
                let new_node = Node::new(value).as_ptr();
                unsafe {
                    let next = (*prev.as_mut()).next;
                    (*prev.as_mut()).next = NonNull::new(new_node);
                    (*new_node).next = next;
                };
            }
            Ok(())
        }
    }
}
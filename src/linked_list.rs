// use std::alloc::{alloc, dealloc, Layout};
// use std::mem::MaybeUninit;
use std::ptr::{NonNull};
use std::usize;

#[derive(Debug)]
pub enum LinkedListError {
    IndexError(usize),
    EmptyListError,
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
    unsafe fn get(ptr: NonNull<Node<T>>) -> T {
        let b = Box::from_raw(ptr.as_ptr());
        let Node{ now, .. } = *b;
        now
    }
    unsafe fn destroy(ptr: NonNull<Node<T>>) {
        drop(Box::from_raw(ptr.as_ptr()));
    }
}

pub struct LinkedList<T> {
    ptr: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
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
                let mut new_node = Node::new(value);
                unsafe {
                    (*new_node.as_mut()).next = node
                };
                self.ptr = NonNull::new(new_node.as_ptr());
            } else {
                let mut prev = node.ok_or(LinkedListError::IndexError(idx))?;
                for _ in 0..(idx - 1) {
                    unsafe {
                        prev = (*prev.as_ptr()).next.ok_or(LinkedListError::IndexError(idx))?
                    };
                }
                let mut new_node = Node::new(value);
                unsafe {
                    let next = (*prev.as_ref()).next;
                    (*prev.as_mut()).next = NonNull::new(new_node.as_ptr());
                    (*new_node.as_mut()).next = next;
                };
            }
            self.len += 1;
            Ok(())
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), LinkedListError> {
        let node = Node::new(value);
        if self.size() == 0 {
            self.ptr = Some(node);
        } else {
            let mut prev = self.ptr.ok_or(LinkedListError::EmptyListError)?;
            for i in 0..(self.size() - 1) {
                unsafe {
                    prev = (*prev.as_ptr()).next.ok_or(LinkedListError::IndexError(i))?
                };
            }
            let node_ptr = node.as_ptr();
            unsafe {
                (*prev.as_mut()).next = NonNull::new(node_ptr);
            }
        }
        self.len += 1;
        Ok(())
    }

    pub fn remove(&mut self, idx: usize) -> Result<T, LinkedListError> {
        if self.is_empty() {
            Err(LinkedListError::IndexError(idx))
        } else {
            if idx == 0 {
                unsafe {
                    let target = self.ptr.unwrap().as_ptr();
                    let next = (*target).next;
                    self.ptr = next;
                    self.len -= 1;
                    Ok(Node::get(NonNull::new(target).unwrap()))
                }
            } else {
                unsafe {
                    let mut prev = self.ptr.unwrap();
                    for _ in 0..(idx - 1) {
                        prev = (*prev.as_ref()).next.ok_or(LinkedListError::IndexError(idx))?;
                    }
                    let target = (*prev.as_ref()).next.ok_or(LinkedListError::IndexError(idx))?;
                    let next = (*target.as_ref()).next;
                    (*prev.as_mut()).next = next;
                    self.len -= 1;
                    Ok(Node::get(target))
                }
            }
        }
    }

    pub fn pop(&mut self) -> Result<T, LinkedListError> {
        if self.is_empty() {
            Err(LinkedListError::EmptyListError)
        } else {
            if self.size() == 1 {
                if let Some(ptr) = self.ptr {
                    self.len -= 1;
                    self.ptr = None;
                    unsafe {
                        Ok(Node::get(ptr))
                    }
                } else {
                    Err(LinkedListError::EmptyListError)
                }
            } else {
                let mut prev = self.ptr.unwrap();
                unsafe {
                    for _ in 0..(self.size() - 2) {
                        prev = (*prev.as_ref()).next.ok_or(LinkedListError::EmptyListError)?;
                    }
                    let target = (*prev.as_ref()).next
                        .ok_or(LinkedListError::IndexError(self.size() - 1))?;
                    (*prev.as_mut()).next = None;
                    self.len -= 1;
                    Ok(Node::get(target))
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while let Some(n) = self.ptr {
            unsafe {
                let next = n.as_ref().next;
                Node::destroy(n);
                self.ptr = next;
            }
        }
        self.ptr = None;
        self.len = 0;
    }
}
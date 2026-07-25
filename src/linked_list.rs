use core::fmt;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

pub enum LinkedListError {
    IndexError(usize),
}

impl std::fmt::Display for LinkedListError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::IndexError(idx) => write!(f, "invalid index: {idx}"),
        }
    }
}

struct Node<T> {
    now: *mut T,
    next: *mut T,
}

impl<T> Node<T> {
    fn new() -> Self {
        Self {
            now: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}
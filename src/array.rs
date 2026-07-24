use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
pub enum ArrayError {
    CapacityOverflow,
    Underflow,
    IndexError(usize),
    LayoutError(std::alloc::LayoutError),
}

impl std::fmt::Display for ArrayError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::CapacityOverflow => write!(f, "capacity overflow"),
            Self::Underflow => write!(f, "stack underflow"),
            Self::IndexError(index) => write!(f, "IndexError: {index} is invalid index"),
            Self::LayoutError(layout_error) => write!(f, "LayoutError: {layout_error}"),
        }
    }
}

impl std::error::Error for ArrayError {}

impl From<std::alloc::LayoutError> for ArrayError {
    fn from(value: std::alloc::LayoutError) -> Self {
        ArrayError::LayoutError(value)
    }
}

#[derive(Debug)]
pub struct IntArray {
    ptr: *mut i32,
    capacity: usize,
    len: usize,
}

impl Array
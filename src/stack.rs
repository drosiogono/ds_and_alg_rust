use std::alloc::{alloc, dealloc, Layout};
// use std::ptr;

#[derive(Debug)]
pub enum StackError {
    CapacityOverflow,
    Underflow,
    LayoutError(std::alloc::LayoutError),
}

impl std::fmt::Display for StackError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::CapacityOverflow => write!(f, "capacity overflow"),
            Self::Underflow => write!(f, "stack underflow"),
            Self::LayoutError(layout_error) => write!(f, "LayoutError: {layout_error}"),
        }
    }
}

impl std::error::Error for StackError {}

impl From<std::alloc::LayoutError> for StackError {
    fn from(value: std::alloc::LayoutError) -> Self {
        StackError::LayoutError(value)
    }
}

#[derive(Debug)]
pub struct IntStack<const N: usize> {
    ptr: *mut i32,
    top: usize,
    capacity: usize,
}

impl<const N: usize> IntStack<N> {
    pub fn new() -> Result<Self, StackError> {
        let layout = Layout::array::<i32>(N)?;

        let ptr = unsafe {
            alloc(layout) as *mut i32
        };

        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        Ok(Self {
            ptr,
            top: 0,
            capacity: N,
        })
    }

    fn grow(&mut self) -> Result<(), StackError> {
        let new_capacity = self.capacity.checked_add(N).ok_or(StackError::CapacityOverflow)?;
        let old = Layout::array::<i32>(self.capacity)?;
        let new = Layout::array::<i32>(new_capacity)?;

        let new_ptr = unsafe {
            alloc(new) as *mut i32
        };

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new)
        };

        unsafe {
            for i in 0..self.top {
                let value = self.ptr.add(i).read();
                new_ptr.add(i).write(value);
            }

            dealloc(
                self.ptr as *mut u8,
                old,
            );
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
        Ok(())
    }

    pub fn push(&mut self, value: i32) -> Result<(), StackError>{
        if self.top == self.capacity {
            self.grow()?;
        }
        unsafe {
            self.ptr.add(self.top).write(value);
        }
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<i32, StackError>{
        if self.top == 0 {
            Err(StackError::Underflow)
        } else {
            self.top -= 1;
            let value = unsafe {
                self.ptr.add(self.top).read()
            };
            Ok(value)
        }
    }

    pub fn peek(&self) -> Result<i32, StackError> {
        if self.top == 0 {
            Err(StackError::Underflow)
        } else {
            let value = unsafe {
                self.ptr.add(self.top - 1).read()
            };
            Ok(value)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

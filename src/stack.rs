use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum StackError {
    CapacityOverflow,
    // InvalidCapacity,
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
            // Self::InvalidCapacity => write!(f, "invalid capacity"),
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
pub struct Stack<T> {
    ptr: *mut T,
    top: usize,
    capacity: usize,
    phantom: PhantomData<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            top: 0,
            capacity: 0,
            phantom: PhantomData,
        }
    }

    fn grow(&mut self) -> Result<(), StackError> {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity.checked_mul(2).ok_or(StackError::CapacityOverflow)?
        };
        let old = Layout::array::<T>(self.capacity)?;
        let new = Layout::array::<T>(new_capacity)?;

        let new_ptr = unsafe {
            alloc(new) as *mut T
        };

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new)
        };

        unsafe {
            for i in 0..self.top {
                let value = self.ptr.add(i).read();
                new_ptr.add(i).write(value);
            }

            if !self.ptr.is_null() {
                dealloc(
                    self.ptr as *mut u8,
                    old,
                );
            }
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
        Ok(())
    }

    pub fn push(&mut self, value: T) -> Result<(), StackError>{
        if self.top == self.capacity {
            self.grow()?;
        }
        unsafe {
            self.ptr.add(self.top).write(value);
        }
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, StackError>{
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

    pub fn peek(&self) -> Result<&T, StackError> {
        if self.top == 0 {
            Err(StackError::Underflow)
        } else {
            let value = unsafe {
                &*self.ptr.add(self.top - 1)
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

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.top {
                ptr::drop_in_place(self.ptr.add(i));
            }
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

impl<T, const N: usize> From<[T; N]> for Stack<T> {
    fn from(s: [T; N]) -> Self {
        if std::mem::size_of::<T>() == 0 {
            panic!("Stack does not support zero-sized types.");
        }
        if N == 0 {
            Self::new()
        } else {
            let capacity = N;
            let top = N;
            let layout = Layout::array::<T>(capacity).expect("Layout error");
            let ptr = unsafe {
                alloc(layout) as *mut T
            };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            for (i, v) in s.into_iter().enumerate() {
                unsafe {
                    ptr.add(i).write(v)
                };
            }
            Self {
                ptr,
                top,
                capacity,
                phantom: PhantomData,
            }
        }
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(s: Vec<T>) -> Self {
        if std::mem::size_of::<T>() == 0 {
            panic!("Stack does not support zero-sized types.");
        }
        if s.len() == 0 {
            Self::new()
        } else {
            let capacity = s.len();
            let top = s.len();
            let layout = Layout::array::<T>(capacity).expect("Layout error");
            let ptr = unsafe {
                alloc(layout) as *mut T
            };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            for (i, v) in s.into_iter().enumerate() {
                unsafe {
                    ptr.add(i).write(v)
                };
            }
            Self {
                ptr,
                top,
                capacity,
                phantom: PhantomData,
            }
        }
    }
}
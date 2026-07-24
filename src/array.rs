use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::ops::{Index, IndexMut};

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
            Self::IndexError(index) => write!(f, "IndexError: {index} is invalid"),
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
pub struct Array<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
}

impl<T, const N: usize> From<[T; N]> for Array<T> {
    fn from(s: [T; N]) -> Self {
        if std::mem::size_of::<T>() == 0 {
            panic!("Array does not support zero-sized types.");
        } else if N == 0 {
            Self::new()
        } else {
            let len = N;
            let layout = Layout::array::<T>(len).unwrap();
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
                capacity: N,
                len,
            }
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(
                    self.ptr.add(i)
                );
            }
        }
        if !self.ptr.is_null() {
            let layout = Layout::array::<T>(self.capacity).expect("LayoutError");
            unsafe {
                dealloc(
                    self.ptr as *mut u8,
                    layout
                )
            };
        }
    }
}

impl<T> std::fmt::Display for Array<T> 
    where T: std::fmt::Display {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> fmt::Result {
            write!(f, "[")?;
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self[i])?;
            }
            write!(f, "]")
        }
}

impl<T> Array<T> {
    pub fn new() -> Self {
        if std::mem::size_of::<T>() == 0 {
            panic!("Array does not support zero-sized types.");
        }
        Self {
            ptr: ptr::null_mut(),
            capacity: 0,
            len: 0,
        }
    }

    fn grow(&mut self) -> Result<(), ArrayError> {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity
                .checked_mul(2)
                .ok_or(ArrayError::CapacityOverflow)?
        };
        let old_layout = Layout::array::<T>(self.capacity)?;
        let new_layout = Layout::array::<T>(new_capacity)?;
        let new_ptr = unsafe {
            alloc(new_layout) as *mut T
        };
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }
        
        unsafe {
            for i in 0..self.size() {
                new_ptr.add(i).write(
                    self.ptr.add(i).read()
                );
            }
            if !self.ptr.is_null() {
                dealloc(
                    self.ptr as *mut u8,
                    old_layout,
                )
            }
        }
        self.ptr = new_ptr;
        self.capacity = new_capacity;
        Ok(())
    }

    pub fn insert(&mut self, idx: usize, value: T) -> Result<(), ArrayError> {
        if idx > self.size() {
            Err(ArrayError::IndexError(idx))
        } else {
            if self.is_full() {
                self.grow()?;
            }
            for i in (idx..self.size()).rev() {
                unsafe {
                    self.ptr.add(i + 1).write(
                        self.ptr.add(i).read()
                    )
                };
            }
            unsafe {
                self.ptr.add(idx).write(value)
            };
            self.len += 1;
            Ok(())
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), ArrayError> {
        // if self.is_full() {
        //     self.grow()?;
        // }
        // unsafe {
        //     self.ptr.add(self.size()).write(value)
        // };
        // self.len += 1;
        // Ok(())
        self.insert(self.size(), value)
    }

    pub fn remove(&mut self, idx: usize) -> Result<T, ArrayError> {
        if self.is_empty() {
            Err(ArrayError::Underflow)
        } else if idx >= self.size() {
            Err(ArrayError::IndexError(idx))
        } else {
            let value = unsafe {
                self.ptr.add(idx).read()
            };
            for i in (idx+1)..self.size() {
                unsafe {
                    self.ptr.add(i - 1).write(
                        self.ptr.add(i).read()
                    )
                };
            }
            self.len -= 1;
            Ok(value)
        }
    }

    pub fn pop(&mut self) -> Result<T, ArrayError> {
        if self.is_empty() {
            Err(ArrayError::Underflow)
        } else {
            self.len -= 1;
            let value = unsafe {
                self.ptr.add(self.size()).read()
            };
            Ok(value)
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.size() {
            None
        } else {
            let value: &T = unsafe {
                &*self.ptr.add(idx)
            };
            Some(value)
            // unsafe { self.ptr.add(idx).as_ref() }
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= self.size() {
            None
        } else {
            let value = unsafe {
                &mut *self.ptr.add(idx)
            };
            Some(value)
        }
    }

    pub fn size(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx).expect("index out of bounds")
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.get_mut(idx).expect("index out of bounds")
    }
}
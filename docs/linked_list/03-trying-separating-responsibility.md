I thought:
Instead of LinkedList making and destroying Node, how about letting Node managing its own memory?

Structure:
LinkedList
    ↓
asking Node creation
    ↓
Node allocates T
    ↓
Node destructs T

(Something about representation and responsibility)

Important discovery:
Using `value: *mut T` and Node managing allocation is not the same chocice.
They are separated.

E.G.:
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

We can still:
impl<T> Node<T> {
    fn new(value: T) -> *mut Self {
        // allocate Node<T> itself on heap
    }
}

Data expression: value: T
allocation responsibility: Node

Important separation!
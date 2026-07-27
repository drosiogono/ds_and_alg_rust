I found `Box<T>`.

At first I thought that a Box made from `Box::new` will drop when a function ends because it is a local variable.
fn create() -> *mut Node<T> {
    let node = Box::new(...);

    // node drop?
}

But I found:
Box::into_raw(node)

Box<Node<T>>
        ↓
Box::into_raw
        ↓
*mut Node<T>

Now I can move the ownership of Box to a raw pointer.
So a Node can live beyong the function.

Box
  ↓
raw pointer
  ↓
Box

Creation:
let ptr = Box::into_raw(Box::new(node));

Deletion:
drop(Box::from_raw(ptr));


Important point!
What Box<T> solves:
allocation
deallocation
correct layout
drop glue
ownership abstraction

It helps managing T's lifetime and allocation.

What Box<T> doesn't solve:
If:
```
struct Node<T> {
    value: Box<T>,
    next: *mut Node<T>,
}
```

Node allocation and T allocation still occur twice.

The fundamental solution: Returning to simple modeling
```
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}
```

And allocate the whole Node in heap:
`Box<Node<T>>`
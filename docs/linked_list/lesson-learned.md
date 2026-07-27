# Lessons Learned

## 1. Data representation and allocation strategy are separate decisions

I initially connected these two questions:

```text
Where is T stored?
```

and:

```text
Who allocates the memory?
```

But they are independent.

These are different designs:

```rust
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}
```

and:

```rust
struct Node<T> {
    value: Box<T>,
    next: *mut Node<T>,
}
```

The first stores `T` directly inside the node.

The second stores a pointer to a separately allocated `T`.

Both can be heap-allocated as nodes.

Therefore:

```text
value: T
```

does not imply:

```text
T lives on the stack
```

and:

```text
value: *mut T
```

is not necessary merely because the node must live on the heap.

---

## 2. A raw pointer is not an ownership model

A raw pointer tells me where an object may be located.

It does not tell me:

* who owns the object
* whether the object is still alive
* whether it has already been freed
* whether another pointer also owns the same allocation
* how it should be destroyed

This means that a raw-pointer data structure must maintain its own invariants.

---

## 3. `Box` can be used as an ownership boundary

A useful pattern is:

```text
Box
  ↓
raw pointer
  ↓
data structure operations
  ↓
raw pointer
  ↓
Box
  ↓
drop
```

The data structure can manipulate raw pointers directly while using `Box` to establish and terminate ownership.

This does not eliminate unsafe code.

It reduces the amount of manual memory-management logic that must be written.

---

## 4. Higher-level abstractions are not the opposite of low-level understanding

I initially thought that using `Box` might mean avoiding the low-level implementation.

After implementing allocation and deallocation manually in other data structures, I came to see `Box` differently.

Using `Box` does not require ignorance of what happens underneath.

Rather, understanding `alloc` and `dealloc` makes it possible to understand what responsibility is being delegated to `Box`.

The important question is not:

> Can I implement this manually?

but:

> Which part should I implement manually, and which part should be delegated to an abstraction?

---

## 5. The memory model of the data structure should guide the abstraction

An array-like structure such as Stack, Queue, Array requires:

```text
one contiguous region of memory
```

This makes manual allocation and raw memory management useful.

A linked list consists of:

```text
independent Node objects connected by links
```

This makes allocating individual nodes as objects a natural model.

Therefore, different data structures may justifiably use different levels of abstraction.

There is no rule that every data structure must use `alloc` directly merely because one data structure did.

---

## 6. The most important design question is ownership

For every allocation, I should be able to answer:

```text
Who owns this memory?
```

For the current linked-list design:

```text
Node<T>
    owns T

The list
    maintains the collection of nodes

Raw pointers
    represent links between nodes

Box
    establishes and releases ownership of a Node allocation
```

This ownership model is more important than the syntax used to allocate the memory.

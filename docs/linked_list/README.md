# Linked List

A linked list implemented from first principles in Rust.

The goal of this implementation is not merely to reproduce a linked list, but to investigate the relationship between:

* ownership
* heap allocation
* raw pointers
* object lifetime
* destruction and deallocation
* memory layout
* cache locality
* abstraction boundaries

## Design Evolution

The implementation evolved through several stages.

### Version 1 — A Node Owns Its Value

```rust
struct Node<T> {
    value: T,
    next: ...
}
```

The initial model was conceptually simple: a node directly owns its value.

The main question was not how to represent the value, but how to allocate and manage the lifetime of the node itself.

---

### Version 2 — The Node Stores a Pointer to `T`

```rust
struct Node<T> {
    now: *mut T,
    next: *mut Node<T>,
}
```

The value was separated from the node and allocated independently.

This design came from a desire to make the node responsible for dynamically allocating and destroying its own value.

It also reflected my previous experience implementing data structures using:

```text
alloc
→ raw pointer
→ ptr::write
→ ptr::read / drop_in_place
→ dealloc
```

This design was later reconsidered because it introduced:

* an additional heap allocation for every element
* an additional pointer indirection
* worse memory locality
* more complicated ownership and destruction logic

---

### Version 3 — Manual Allocation

The node itself was allocated using `std::alloc`.

This provided direct control over:

* `Layout`
* allocation
* initialization
* destruction
* deallocation

However, it also meant that correctness depended on manually maintaining several invariants.

---

### Version 4 — `Box` as the Allocation Boundary

The final design uses `Box` to allocate and deallocate a `Node<T>` while using raw pointers for the linked-list structure itself.

Conceptually:

```text
Box<Node<T>>
      ↓
Box::into_raw
      ↓
*mut Node<T>
      ↓
linked-list operations
      ↓
Box::from_raw
      ↓
drop
```

The linked list still operates using raw pointers, but the low-level allocation and deallocation of a complete `Node<T>` are delegated to `Box`.

This creates a useful separation:

```text
Box
    owns allocation and destruction

LinkedList
    owns the relationship between nodes
```

## Current Design

```rust
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}
```

A `Node<T>` directly owns its value.

The list stores raw pointers to nodes, while node creation and destruction use `Box` as the ownership boundary.

## Main Lessons

The most important lesson was that these are separate design questions:

1. Where is `T` stored?
2. Who owns `T`?
3. Who allocates the node?
4. Who destroys the node?
5. How are nodes connected?
6. Which responsibilities should be handled manually?
7. Which responsibilities can safely be delegated to an abstraction?

A major design improvement occurred when the representation of data and the responsibility for allocation were separated conceptually.

`value: T` and `Box<Node<T>>` are not contradictory.

A node can directly own its value while the node itself is heap-allocated and managed through `Box`.

Timeline:
1. Node should own T.
        ↓
2. Node should live when a function ends.
        ↓
3. I need heap allocation.
        ↓
4. From Stack, Queue, Array implementation experience,
I know how to manually use alloc/raw pointer.
        ↓
5. Then how about allocating T separately in heap and letting Node point it?
        ↓
6. But then I need two allocation.
        ↓
7. It is more effective and intuitive to allocate T and Node together.
        ↓
8. I can allocate Node itself in heap with Box.
        ↓
9. I can move the ownership of Node to raw pointer using Box::into_raw.
        ↓
10. While implementing the structure of linked list using raw pointer, I can delegate the allocation and destruction to Box.
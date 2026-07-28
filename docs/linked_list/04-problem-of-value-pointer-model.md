1. Double Allocation
In this Structure:
struct Node<T> {
    now: *mut T,
    next: *mut Node<T>,
}

For one Node<T>, We need two allocation:
alloc(Node<T>)
alloc(T)

N -> 2N!

In contrast, in this structure:
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

Just N allocation.

Each allocation has internal processes in allocator 
-> These may occur:
allocation
pointer
indirection
deallocation

What I realized:
It solved lifetime problem for Node to allocate T separately in heap, but it adds additional memory processes like unnecessary allocation and indirection

2. Indirection
Look at these two structures:
A
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}
memory:
┌──────────────────────┐
│ T                    │
│ next pointer         │
└──────────────────────┘

B
struct Node<T> {
    now: *mut T,
    next: *mut Node<T>,
}
memroy:
Node                    T
┌──────────────┐        ┌──────┐
│ *mut T ───────┼───────>│  T   │
│ *mut Node     │        └──────┘
└──────────────┘

B has additional indirection:

Node access
    ↓
read pointer `now`
    ↓
move to T's own address
    ↓
read T

If Node directly includes T, at least additional pointer dereference for reading T in Node becomes unnecessary.

3. Danger of manual Drop
What I implemented Drop trait for Node<T>:
```
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        self.now.drop_in_place();

        let layout = Layout::new::<T>();

        dealloc(
            self.now as *mut u8,
            layout
        );
    }
}
```

Potential dangers:

- Layout mismatch
dealloc requires the exactly same layout with the layout used in allocation.

alloc(layout A)
        ↓
dealloc(layout B)

Becomes danger.

- Dangling pointer
ptr != null
doesn't assert ptr is safe.

null pointer ≠ dangling pointer

- Double free
destroy(ptr);
destroy(ptr);
When this occurs, it frees the address that's already freed.

- Tracking Ownership
Raw pointer doesn't show who is the owner.
E.G.:
let p1 = ptr;
let p2 = p1;
a pointer may be copied.

Then:
p1 ─┐
    ├──> Node
p2 ─┘
It's hard to find out which one should destroy a Node.

I had to implement very well while satisfying all the warning points.
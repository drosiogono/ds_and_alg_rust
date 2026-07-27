Next structure:
```
struct Node<T> {
    now: *mut T,
    next: *mut Node<T>,
}
```

Reason:
T is stored in Node
        ↓
T is also in Node when Node is created
        ↓
Node itself should be on heap for Node's lifetime to be beyond function

What I already knew:
alloc
↓
*mut T


So I came up with:

T
↓
heap allocation
↓
*mut T

Result:
Node
┌─────────────┐
│ *mut T ─────┼────> T
│ *mut Node   │
└─────────────┘

The reason why I came up with `alloc`(manual heap allocation):
I implemented Stack, Queue, Array before.
Memory procedure:
Layout
  ↓
alloc
  ↓
raw pointer
  ↓
ptr::write
  ↓
ptr::read
  ↓
drop_in_place
  ↓
dealloc

Key concepts I learned with Stack, Queue, Array:
allocation
initialization
ownership
destruction
deallocation
Layout
raw pointer
unsafe
Drop

-> In LinkedList, something like
```
let layout = Layout::new::<Node<T>>();
let ptr = alloc(layout) as *mut Node<T>;
```
The most natural model:
```
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}
```

Memory:

Node<T>
┌────────────────────┐
│ value: T           │
│ next: *mut Node<T> │
└────────────────────┘

Important point:
Node owns T <-- different! --> Node lives on heap

First:
value: T
-> T has local variable lifetime... However...

With `Box<Node<T>>`
Node<T> is now on heap


"Node owns T" and "Node lives on heap" is simultaneously possible
Both are different!
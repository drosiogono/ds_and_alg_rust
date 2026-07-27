To define a linked list: we need Value and Next

Simple modeling:
```
struct Node<T> {
    value: T,
    next: ...,
}
```

Problem: when it's created in a function, it drops when the function ends.

We need to let Node<T> live beyond a function
-> It should be allocated on heap
-> Raw pointer and dynamic allocation.
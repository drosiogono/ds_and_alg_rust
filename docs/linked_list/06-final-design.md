Use Box<T>

Creation:
let node = Box::new(Node::new(...));
let ptr = Box::into_raw(node);

Now I get *mut Node<T> with simpler processes!

Deletion:
let ptr: *mut Node<T> = ...;
let node = Box::from_raw(ptr);
drop(node);

<Summary>
Node<T>
    owns T

LinkedList
    owns the logical collection of nodes

Raw pointer
    represents a link, not ownership by itself

Box
    is used at allocation/deallocation boundaries
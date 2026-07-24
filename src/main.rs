mod stack;
mod queue;
mod array;

use std::alloc::{alloc, dealloc, Layout};
use stack::Stack;
use queue::Queue;
use array::Array;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Trying Queue..");
    let mut queue = Queue::new();
    queue.enqueue(String::from("hola"))?;
    queue.enqueue(String::from("ola"))?;
    queue.enqueue(String::from("Hej"))?;
    for _ in 0..3 {
        println!("{:?}", queue.peek()?);
    }
    println!("size: {}", queue.size());
    for _ in 0..3 {
        println!("{:?}", queue.dequeue()?);
    }
    println!("size: {}", queue.size());
    println!("{}", queue.is_empty());
    println!("{:?}", queue);
    // println!("Provoking an error..");
    // queue.dequeue()?;

    println!("\nTrying Stack..");
    let mut stack = Stack::new();
    stack.push(String::from("hi"))?;
    stack.push(String::from("hello"))?;
    stack.push(String::from("hola"))?;
    for _ in 0..3 {
        println!("{:?}", stack.peek()?);
    }
    println!("size: {}", stack.size());
    for _ in 0..3 {
        println!("{:?}", stack.pop()?);
    }
    println!("size: {}", stack.size());
    println!("{}", stack.is_empty());
    println!("{:?}", stack);

    println!("\nTrying Array..");
    let mut array = Array::new();
    array.push(String::from("hi"))?;
    array.push(String::from("hello"))?;
    array.push(String::from("hola"))?;
    array.insert(0, String::from("ciao"))?;
    println!("size: {}", array.size());
    for i in 0..array.size() {
        println!("array[{i}] = {}", array[i])
    }
    array.remove(2)?;
    for i in 0..array.size() {
        println!("array[{i}] = {}", array[i]);
    }
    array[1] = String::from("안녕하세요");
    for _ in 0..3 {
        println!("pop() -> {}", array.pop()?);
    }
    println!("size: {}", array.size());
    println!("is array empty? {}", array.is_empty());
    println!("{:?}", array);

    println!("\nTrying another Array..");
    let a = [String::from("a"), String::from("b"), String::from("c")];
    let mut array2 = Array::from(a);
    println!("array2 = {}", array2);
    println!("popped: {}", array2.pop()?);
    array2.remove(0)?;
    println!("final array2 = {}", array2);

    println!("\nRust memory alloc/dealloc..");
    let layout = Layout::array::<i64>(4)?;
    let start = unsafe {
        alloc(layout) as *mut i64
    };
    let one_step = unsafe {
        start.add(1)
    };
    let _one_step_layout = Layout::array::<i64>(3)?;

    println!("start    : {:p}", start);
    println!("one_step : {:p}", one_step);

    unsafe {
        dealloc(
            one_step.sub(1) as *mut u8,
            layout,
        )
    };

    Ok(())
}

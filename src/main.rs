mod stack;
mod queue;
// mod array;

use std::alloc::{alloc, dealloc, Layout};
use stack::Stack;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut queue = IntQueue::new(2)?;
    // queue.enqueue(5)?;
    // queue.enqueue(6)?;
    // queue.enqueue(7)?;
    // for _ in 0..3 {
    //     println!("{:?}", queue.peek()?);
    // }
    // println!("size: {}", queue.size());
    // for _ in 0..3 {
    //     println!("{:?}", queue.dequeue()?);
    // }
    // println!("size: {}", queue.size());
    // println!("{}", queue.is_empty());
    // println!("{:?}", queue);
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

    // let layout = Layout::array::<i64>(4)?;
    // let start = unsafe {
    //     alloc(layout) as *mut i64
    // };
    // let one_step = unsafe {
    //     start.add(1)
    // };
    // let _one_step_layout = Layout::array::<i64>(3)?;

    // println!("start    : {:p}", start);
    // println!("one_step : {:p}", one_step);

    // unsafe {
    //     dealloc(
    //         one_step.sub(1) as *mut u8,
    //         layout,
    //     )
    // };

    Ok(())
}

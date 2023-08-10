//  Memory
// |     | Heap 
// |     |   Something like Box type
// | b->5|   let b = Box::new(5);
// |     |
// |     |
// |     |
// | b   | Stack
// | x=5 |  let x = 5; kind of data that is known at compile time

#![allow(dead_code)]
use std::mem;

struct Point{
    x: f64,
    y: f64
}

fn origin() -> Point{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap(){
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes, p1 is point on stack", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes, p2 is a box on heap", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("let p3 = *p2; // p2 is relocated to the stack as p3");
    println!("p3.x = {}", p3.x);
    println!("p3 takes up {} bytes, p3 is point on stack", mem::size_of_val(&p3))    
}
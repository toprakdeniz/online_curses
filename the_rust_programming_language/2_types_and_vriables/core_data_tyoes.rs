#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

fn main(){
    let a: u8 = 255;
    println!("a = {}", a);

    let c = 123456789;

    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, size = {} bytes {}-bits OS", z, size_of_z, size_of_z*8); 

    let d: char = 'x';
    let size_of_d = mem::size_of_val(&d);
    println!("d = {}, size = {} bytes", d, size_of_d);

    // f32 f64 IEEE754 signed

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64
    let size_of_e = mem::size_of_val(&e);
    println!("e = {}, size = {} bytes", e, size_of_e);


    let g: bool = false;
    let size_of_g = mem::size_of_val(&g);
    println!("g = {}, size = {} bytes", g, size_of_g);

}
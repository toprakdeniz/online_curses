use std::mem;

fn arrays(){
    let mut a[:i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("does not match");
    }

    let b = [1; 10];
    println!("b is not initilized with explicit type but value of 1")
    println!("b has {} elements, first is {}", b.len(), b[0]);
    let b = [1u16; 10];
    println!("b is initilized with explicit u16 type but value of 1")
    println!("b has {} elements, first is {}", b.len(), b[0]);

    
}

fn main(){
    arrays();
}
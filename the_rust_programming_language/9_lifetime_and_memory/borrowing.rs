fn main(){
    let print_vector = |x:&Vec<i32>| {
        println!("{:?}", x);
    };

    let v = vec![1,2,3];
    print_vector(&v);
    // println!("{:?}", v); // v is borrowed by print_vector

    let mut a = 40;
//    {
     let b = &mut a;
     *b += 2;
//    }// it worked without this scope
    println!("{}", b);
    println!("{}", a);

    let mut z = vec![3,2,1];
    
    for i in &z{
        println!("i = {}", i);
        // z.push(5); // cannot borrow `z` as mutable because it is also borrowed as immutable
    }

}
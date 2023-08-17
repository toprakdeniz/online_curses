fn main(){
    let v = vec![1, 2, 3];
    // for x in vec{ // consumes v
    //     println!("{}", x);
    // }
    for x in &v{ // borrows v
        println!("{}", x);
    }

    for x in v.iter(){ // borrows v
        println!("{}", x);
    }

    // transform a collection into iterator
    for x in v.into_iter(){ // consumes v
        println!("{}", x);
    }

    let mut v = vec![1, 2, 3];
    for x in v.iter_mut(){ // borrows v mutably
        *x += 1;
    }
    println!("{:?}", v);

    // reverse
    for x in v.iter().rev(){
        println!("{}", x);
    }


    let u = 1;
    let u2 = u;
    println!("{}", u);// works because u is Copy. Primitive types are cheap to copy

    
}
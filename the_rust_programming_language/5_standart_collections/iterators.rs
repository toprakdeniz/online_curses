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


    // into_iter more

    let mut vec2 = vec![3,2,1];

    vec2.extend(v); // uses into_iter implicitly
    println!("{:?}", vec2);


    let vec3 = vec![1, 2, 3];
    for x in vec3{
        if x == 1{ break;}
    }
    // println!("{:?}", vec3); // error: use of moved value: `vec3`

{}
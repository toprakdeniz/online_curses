fn print_value(v: &Vec<i32>, idx: usize){
    match v.get(idx){
        Some(x) => println!("vector[{}] = {}", idx, x),
        None => println!("error, no such element"),
    };
}

fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);


    let idx: usize = 0;
    print_value(&a, idx);
    let idx: usize = 111;
    print_value(&a, idx);


    // pop element  

    a.push(42);
    println!("a = {:?}", a);
    let last_elem = a.pop(); // Option
    println!("last elem = {:?}, a = {:?}", last_elem, a);

    // let Some(x) = a.pop(); // error: expected tuple struct/variant, found struct `std::option::Option`


    while let Some(x) = a.pop(){ // This works! As long as a.pop() gives a Some(x) value, the loop will continue
        println!("{}", x);
    }

}

fn main(){
    vectors();
}
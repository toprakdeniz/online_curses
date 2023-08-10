fn shadowing(){
    let a = 123;
    println!("a = {}", a);

    let a = 456;
    println!("a = {}", a);
}

fn scoping(){
    let a = 123;
    println!("a = {}", a);

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }
    
    // println!("outside, b = {}", b)
    println!("outside, a = {}", a);
}

fn main(){
    println!("shadowing");
    shadowing();
    println!("scoping");
    scoping();
}
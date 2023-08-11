fn sum_and_product(x: i32, y: i32) -> (i32, i32){
    (x + y, x * y)
}

fn tuples(){
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("sp = ({}, {})", sp.0, sp.1);
    let combined = (sp, x, y);
    println!("combined = {:?}", combined);
    // destructuring
    println!("last element of tuple at the start: {}", combined.0.1);
    let ((a, b), c, d) = combined;
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    let foo = (true, 42.0, -1i8);
    println!("foo = {:?}", foo);
    let meaning = (42,);
    println!("meaning = {:?}", meaning);
}

fn main(){
    tuples();
}
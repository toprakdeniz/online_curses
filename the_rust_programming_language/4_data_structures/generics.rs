struct Point<T>{
    x: T,
    y: T,
}

fn generics(){
    let a: Point<i32> = Point{x:0, y:0};
    let b = Point{x:1.2, y:3.4};
    println!("a = ({}, {})", a.x, a.y);
    println!("b = ({}, {})", b.x, b.y);
}

fn main(){
    generics();
}
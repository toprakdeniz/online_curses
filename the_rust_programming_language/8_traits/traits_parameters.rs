use std::fmt::Debug;

// fn print_info(shape: impl Shape + Debug) 
// fn print_info<T: Shape + Debug>(shape: T)
fn print_info<T>(shape: T)
where 
    T: Shape + Debug,
{
    println!("{:?}", shape);
    println!("area: {}", shape.area());
}

trait Shape{
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Circle{
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

fn main(){
    let rect = Rectangle { width: 3.0, height: 4.0 };
    let circle = Circle { radius: 5.0 };

    print_info(rect);
    print_info(circle);
}
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

fn origin() -> Point {
    Point { x: 0, y: 0 }
}

fn structures(){
    let p = Point(x: 10, y: 20);
    println!("The point is at ({}, {})", p.x, p.y);

    let my_line = Line { start: origin(), end: Point { x: 10, y: 10 } };

}   

fn main(){
    structures();
}
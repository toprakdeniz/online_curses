use std::mem;

trait Printable{
    fn format(&self) -> String;
}


impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String{
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// this is expensive
fn print(value: &Printable) { //
    println!("{}", value.format());
}
// monomorphisation
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

fn main(){
    let a = 123;
    let b = "hello".to_string();

    println!("{}", a.format());
    println!("{}", b.format());

    print(&a);// the information that a is an i32 is lost
    print(&b);// this is dynamic dispatch. because it needs to look at the type and call particular function

    print_it(a);
    print_it(b);

    print_it(a);    
}
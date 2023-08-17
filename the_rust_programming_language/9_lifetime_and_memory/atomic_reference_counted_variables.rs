use std::rc::Rc;
use std::sync::Arc;
use std::thread;


struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn arc_demo(){
    let name = Arc::new("John".to_string());
    let person = Person::new(name);
    let t = thread::spawn( move || {
        person.greet();
    });

    t.join().unwrap();
}


fn main(){
    arc_demo();
}
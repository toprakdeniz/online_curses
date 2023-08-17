use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;


struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person {
            name: name,
            state: state
        }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
        let mut self_state = self.state.lock().unwrap();
        self_state.clear();
        self_state.push_str("excited");
        println!("I am {}", self_state);

    }
}


fn mutex_demo(){
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("happy".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn( move || {
        person.greet();
    });
    println!("Name = {}, name has {} strong pointers", name, Arc::strong_count(&name));
    t.join().unwrap();
}



fn main(){
    mutex_demo();
}
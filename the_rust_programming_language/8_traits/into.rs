struct Person {
    name: String,
}

impl Person {
    // fn new<S: Into<String>>(name: S) -> Person 
    fn new<S>(name: S) -> Person 
    where 
        S: Into<String>,
    {
        Person { name: name.into() }
    }
}

struct Person2 {
    name: String,
}

impl Person2 {
    fn new( name: &str)-> Person2 {
        Person2 { name: name.to_string() }
    }

    fn show(&self) {
        println!("name: {}", self.name);
    }
}


fn main(){
    let john = Person::new("John");
    let name: String = "Jane".to_string();
    let jane = Person::new(name);

    let john2 = Person2::new("John");
    john2.show();
    let name2: String = "Jane".to_string();
    let jane2 = Person2::new(&name2);
    jane2.show();

    let name3: String = "Jane".to_string();
    let jane3 = Person2::new(name3.as_str());
    jane3.show();

}
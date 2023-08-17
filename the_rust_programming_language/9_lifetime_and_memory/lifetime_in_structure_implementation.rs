struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn main(){
    let boss = Person {name: "Elon Musk"};
    boss.talk();
}
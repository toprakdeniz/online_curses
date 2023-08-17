use std::fmt::Display;


struct Creature {
    name: String
}


impl Creature {
    fn new<S>(name: S) -> Creature 
    where 
        S: Into<String> + Display,
    {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}


impl Drop for Creature{
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn main() {

    let goblin = Creature::new("Jeff");
    println!("Game proceeds");
    drop(goblin);
    println!("Game proceeds");

    let clever: Creature;
    {
        let goblin = Creature::new("clever");
        clever = goblin;
        println!("Game proceeds");
    }

    println!("scope ends");
}
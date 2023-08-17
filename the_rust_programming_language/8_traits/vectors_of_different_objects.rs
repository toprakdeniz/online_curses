trait Animal{
    fn name(&self) -> &'static str;
    fn talk(&self){
        println!("{} cannot talk", self.name());
    }
}

struct Human{
    name: &'static str,
}

struct Cat{
    name: &'static str,
}

impl Animal for Human{
    fn name(&self) -> &'static str{
        self.name
    }

    fn talk(&self){
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat{


    fn name(&self) -> &'static str{
        self.name
    }

    fn talk(&self){
        println!("{} says meow", self.name());
    }
}

// 1. way
enum Creature{
    Human(Human),
    Cat(Cat),
}


fn main(){
    // let mut creatures = Vec::new();
    let mut creatures = Vec::new();
    // creatures.push(Human::create("John"));
    // creatures.push(Cat::create("Kitty"));


    //1.way ..  vector of enum
    creatures.push(Creature::Human(Human{name:"John"}));
    creatures.push(Creature::Cat(Cat{name:"Kitty"}));
    for c in creatures{
        match c{
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }
    // downsides of this way:
    // we need an enum
    // we need to match

    // 2. way .. vector of trait objects
    // let mut animals: Vec<Animal> = Vec::new(); // this does not work becuase Animal does not have a fixed size
    let mut animals:Vec<Box<dyn Animal>> = Vec::new(); // so we but them in a box
    animals.push(Box::new(Human{name:"John"}));
    animals.push(Box::new(Cat{name:"Kitty"}));
    for c in animals{
        c.talk();
    }
}
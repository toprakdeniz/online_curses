trait Animal{
    fn name(&self) -> &'static str;

    fn talk(&self){
        println!("{} cannot talk", self.name());
    }

    fn create(name: &'static str) -> Self;
}

struct Cat {
    name: &'static str
}

struct Human{
    name: &'static str
}

impl Animal for Cat{
    fn create(name: &'static str) -> Cat{
        Cat{name: name}
    }
    
    fn name(&self) -> &'static str{
        self.name
    }

    fn talk(&self){
        println!("{} says meow", self.name());
    }
}

impl Animal for Human{
    fn create(name: &'static str) -> Human{
        Human{name: name}
    }

    fn name(&self) -> &'static str{
        self.name
    }

    fn talk(&self){
        println!("{} says hello", self.name());
    }
}


trait Summable<T>{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32{
        self.iter().fold(0, |a, &b| a + b)
    }
}

fn traits(){

    let h = Human{name: "Deniz"};
    h.talk();
    println!("{}", h.name());

    let c = Cat::create( "Miyav");
    c.talk();
    println!("{}", c.name());

    let h: Human = Animal::create("Deniz");

    let a = vec![1,2,3];
    println!("{} ,{:?}", a.sum(), a);

}

fn main(){
    traits();
}
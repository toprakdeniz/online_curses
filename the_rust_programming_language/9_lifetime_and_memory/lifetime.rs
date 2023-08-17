struct Person{
    name: String,
}

impl Person{
    fn get_ref_name<'a>(&'a self) -> &'a String{ // 'a is implicitly implemented. It is not necessary to write it
        &self.name
    }
}

struct Company<'z>{
    name: String,
    ceo: &'z Person,
}

fn main(){
    // let boss = Person {name: String::from("Elon Musk")};
    // let tesla = Company {name: String::from("Tesla"), ceo: &boss};

    let mut z: &String;
    {
        let p = Person {name: String::from("Elon Musk")};
        z = p.get_ref_name();
        println!("z: {}", z);
    }// doesnt compile because p is dropped here
    println!("z: {}", z);
}

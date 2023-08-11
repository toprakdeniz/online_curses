use std::collections::HashMap;

fn main(){
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    shapes.insert("square".to_string(), 5);
    println!("{:?}", shapes);

    // shapes.entry("circle".to_string()).or_insert(1);
    {
        let actual = shapes.entry("circle".to_string()).or_insert(2);
        println!("{}", actual);
        *actual = 0;
    }
    println!("{:?}", shapes);
}
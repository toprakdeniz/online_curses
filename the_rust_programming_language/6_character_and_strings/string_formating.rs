fn main(){
    let name = "Peter";
    let greeting = format!("Hi, I'm {}", name);
    println!("{}", greeting);


    let hello = "Hello";
    let rust = "Rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let run_forest = format!("{0}, {1}, {0}!", run, forest);
    println!("{}", run_forest);

    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = "delta"
    );
    println!("{}", mixed);
}
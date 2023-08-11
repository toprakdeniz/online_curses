fn string_static(){
    // utf-8
    let s: &'static str = "hello there!"; // &str = string slice
    // static means that this string is stored in the binary of the program

    for c in s.chars(){
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0){
        println!("first letter is {}", first_char);
    }
}


fn string_on_heap(){
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    letters.push_str("!");
    println!("{}", letters);

    // &str <> String // not really works // deref convertion
    let u:&str = &letters;

    // concatenation
    // String + str
    // let z = letters + "abc";
    // let z = letters + &letters; // doesnt work 


}

fn main(){
    string_static();
    string_on_heap();
}
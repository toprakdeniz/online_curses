// Option -> Some(T) or None

fn divide(x: f64, y: f64){
    let mut result: Option<f64> = if y == 0.0 { None } else { Some(x/y) };
    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide {} by {}", x, y),
    };
    if let Some(z) = result { println!("let clause: z = {}", z); }

    while let Some(z) = result { println!("while let clause: z = {}", z); result = None;}
}

fn main(){
    divide(3.0, 2.0);
    divide(3.0, 0.0);
}
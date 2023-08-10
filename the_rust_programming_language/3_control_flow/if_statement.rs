fn if_assignment(){

    let temp = 15;
    let weather = if temp > 20 {
        "sunny"
    } else {
        "cloudy"
    };
    println!("The weather is {}", weather);

    println!("The temprature is {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"warm"});
}

fn main(){
    if_assignment();
}
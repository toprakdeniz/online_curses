// functions takes a function
// and returns a function

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn is_even2(n: &u32) -> bool {
    *n % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}

fn main(){

    // sum of all even squares < 500

    let limit = 500;
    
    let above_limit = greater_than(limit);
    
    let mut sum = 0;
    for i  in 0.. {
        let isq = i*i;
        if above_limit(isq) { break; }
        else if is_even(isq) { sum += isq; }
    }

    println!("loop sum = {}", sum);


    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit) // Takes a reference because it apply condition.
        .filter(|x:&u32| is_even(*x)) //Takes a reference because it apply condition.
        .fold(0, |sum, x| sum + x);


    println!("sum2 = {}", sum2);
}
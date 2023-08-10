fn operators()
{   
    // arithmetic
    let a = 2+3*4;
    
    let a_cubed = i32::pow(a, 3);
    println!("a = {}, a_cubed = {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    let b_to_pi2 = b.powf(std::f64::consts::PI);
    println!("b = {}, b_cubed = {}, b_to_pi = {}, b_to_pi2 = {}", b, b_cubed, b_to_pi, b_to_pi2);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("1|2 = {}", c);

    // bitwise shift
    let two_to_10 = 1 << 10;
    println!("2^10 = {}, 1 << 10", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("pi_less_4 = {}", pi_less_4);
}

fn main()
{
    operators();
}
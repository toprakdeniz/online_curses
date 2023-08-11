fn how_many(x:i32) -> &'static str{
    match x{
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        9..=11 => "lots of",
        _ if (x % 2 == 0) => "even number of",
        _ => "a few"
    }
}


struct CymkColor{
    cyan:u8,
    magenta:u8,
    yellow:u8,
    black:u8,
}

pub fn pattern_matching(){
    for x in 0..13{
        println!("x = {}, I have {} oranges", x, how_many(x));
    }

    let point = (3,4);

    match  point {
        (0,0 ) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("y axis, x = {}", x),
        (x,y) => println!("x = {}, y = {}", x, y),
        // (ref mut x,0) => println!("y axis, x = {}", x),
    }

    let cymk = CymkColor{
        cyan:0,
        magenta:128,
        yellow:0,
        black:0,
    };

    match cymk {
        CymkColor{cyan:_, magenta:128, yellow:_, black:_} => println!("magenta"),
        CymkColor{cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        CymkColor{black:0,..} => println!("it is bright!"),
        _ => println!("other color"),
    }
}

fn main(){
    pattern_matching();
}
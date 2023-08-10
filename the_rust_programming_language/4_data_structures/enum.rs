#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // tuple
    CmykColor{cyan: u8,magenta: u8, yellow: u8,black: u8}, // struct 
}

fn print_color(color:Color){
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGBColor(0, 0, 0) => println!("Black"),
        Color::RGBColor(255, 255, 255) => println!("White"),
        Color::RGBColor(r, g, b) => println!("RGB Color({}, {}, {})", r, g, b),
        Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("Black"),
        Color::CmykColor{cyan, magenta, yellow, black} => println!("CMYK Color({}, {}, {}, {})",cyan, magenta, yellow, black),
    }
}

fn enums(){
    let c: Color = Color::Red;
    print_color(c);
    
    let c: Color = Color::RGBColor(255, 0, 0);
    print_color(c);
    
    let c: Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};
    print_color(c);

    let c: Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 0};
    print_color(c);
}


fn main(){
    enums();
}
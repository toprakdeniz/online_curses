// you dont really know what is inside a union
// unions are primarily used to interface with C or similar languages 
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value( iof: IntOrFloat){
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }
            IntOrFloat { f } => {
                println!("value = {}", f);
            }
        }
    }
}

fn main(){
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    println!("process_value function will treat the integers other than 42 as floats");
    process_value(IntOrFloat { i: 5 });
    process_value(IntOrFloat { f: 42.0 });
    process_value(IntOrFloat { i: 42 });
}
const MEANING_OF_LIFE: u8 = 42; // no fixed address

static Z:i32 = 123; // fixed address

static mut Z_MUTABLE:i32 = 123; // fixed address
// Thread unsafe. Can be changed by multiple threads at the same time.
// Must be used with unsafe block

fn main(){
    println!("{}", MEANING_OF_LIFE);
    // is the same as 
    // println!("{}", 42);

    println!("{}", Z);

    unsafe {
        Z_MUTABLE = 777;
        println!("{}", Z_MUTABLE);
    }

}
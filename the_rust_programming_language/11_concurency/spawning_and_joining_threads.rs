use std::thread;
use std::time::Duration;

fn main(){

    let handle = thread::spawn(|| {
        for _ in 1..10{
            print!("-");
            thread::sleep(Duration::from_millis(500));
        }
    });


    for _ in 1..10{
        print!("|");
        thread::sleep(Duration::from_millis(300));
    }


    handle.join().unwrap();
}
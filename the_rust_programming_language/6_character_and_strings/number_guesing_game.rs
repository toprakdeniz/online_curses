// use rand::Rng;
use std::io::stdin;

fn main(){

    // let number = Rng::thread_rng().gen_range(1,101);
    let number = 40;

    loop{
        println!("Guess the number");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}",guess);
        match guess.cmp(&number){
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

}
fn closures() {
    
    let plus_one = |x: i32| -> i32 { x + 1 };
    assert_eq!(2, plus_one(1));


    let mut two = 2;
    {
    let plus_two = |x| { x + two }; // two is borrowed here
    let mut num = 5;
    println!("{} borrow is took place in a skope destrowed so two can be used", plus_two(num));
    }


    let plus_three = |mut x: i32| x += 3;
    plus_three(two);
    println!("{}", two);


}

fn main(){
    closures();
}
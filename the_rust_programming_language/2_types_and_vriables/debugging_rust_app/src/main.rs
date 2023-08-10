// rust analyzer is installed. Vscode suggested an extention for debugging
// started a project with cargo and opened vscode in the project folder so rust analyzer can work
fn double_value(x: i32) -> i32 {
    x * 2
}

fn main() {
    let mut x = 1;
    x = double_value(x);
    x = 42;
    println!("Hello, world!");
}

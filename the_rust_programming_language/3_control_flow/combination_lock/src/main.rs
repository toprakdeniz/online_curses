use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked,
}


fn main(){
    let code = String::from("1234");

    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                // on vs code press ctrl + read_line to jump to the function
                match stdin().read_line(&mut input){
                    Ok(_) => {
                        entry.push_str(&input.trim_end());// removes end of line
                    }
                    Err(_) => continue,
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                } 
                if !code.starts_with(&entry){
                    state = State::Failed;
                }
            },
            State::Failed => {
                println!("FAILED");
                entry.clear(); // sets entry to ""
                state = State::Locked;
                continue;
            },
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            },
        }
    }

}
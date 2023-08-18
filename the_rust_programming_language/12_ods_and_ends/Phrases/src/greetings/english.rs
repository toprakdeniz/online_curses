//! This module contains English phrases.
//! 
//! # Examples
//! ```
//! extern crate phrases;
//! use phrases::greetings::english;
//! fn main() {
//!    println!("Hello in English: {}", english::hello());
//!    println!("Good bye in English: {}", english::good_bye());
//! }
//! ```

// command to compile and run this file:

/*  
    $ cd the_rust_programming_language/12_ods_and_ends/Phrases
    $ rustc src/main.rs
    $ ./main
*/

/// Contains English phrases.
pub fn hello() -> String {
    "Hello!".to_string()
}


pub fn good_bye() -> String {
    "Good bye!".to_string()
}

//rustdoc src/greetings/english.rs to run and generate documentation
pub mod greetings
{
    pub mod english;
    pub mod turkish {
        pub fn hello() -> String {
            "Merhaba!".to_string()
        }

        pub fn good_bye() -> String {
            "Güle güle!".to_string()
        }
    }
}


#[test] // cargo build -> cargo test
fn english_greetings_correct(){
    assert_eq!("Hello!", greetings::english::hello());
    assert_eq!("Good bye!", greetings::english::good_bye());
}
#[test]
fn turkish_greetings_correct(){
    assert_eq!("Merhaba!", greetings::turkish::hello());
    assert_eq!("Güle güle!", greetings::turkish::good_bye());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn turkish_greetings_correct2(){
    assert_eq!("Selam!", greetings::turkish::hello());
    assert_eq!("Güle güle!", greetings::turkish::good_bye());
}
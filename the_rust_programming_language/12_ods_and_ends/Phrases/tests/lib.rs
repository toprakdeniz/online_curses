#[cfg(test)]
// mod tests { } // not necessary
extern crate phrases;

#[test]
fn english_greetings_correct(){
    assert_eq!("Hello!", phrases::greetings::english::hello());
    assert_eq!("Good bye!", phrases::greetings::english::good_bye());
}
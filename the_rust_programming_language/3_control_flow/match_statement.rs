fn main() {
    let country_code = 90;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        90 => "Republic of Turkey",
        1..=999 => "unknown",
        _ => "invalid" // exhausts the rest of patterns
    };

    println!("The country with code {} is {}", country_code, country);
}


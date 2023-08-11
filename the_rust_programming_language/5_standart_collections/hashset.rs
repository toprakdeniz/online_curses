use std::collections::HashSet;

fn main(){
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");

    println!("{:?}", greeks);

    let vega_inserted = greeks.insert("vega");
    if vega_inserted {
        println!("We added vega!");
    }

    if !greeks.insert("vega") {
        println!("vega is already in the set");
    }

    if greeks.remove("delta") {
        println!("We removed delta!");
    }

    if !greeks.contains("kappa") {
        println!("kappa wasn't in the set, so nothing removed");
    }

    let _1_5 = (1..=5).collect::<HashSet<_>>();
    println!("{:?}", _1_5);

    let _1_3 = (1..=3).collect::<HashSet<_>>();

    // subset
    if _1_3.is_subset(&_1_5) {
        println!("1, 2, 3 is a subset of 1, 2, 3, 4, 5");
    }

    let _6_10 = (6..=10).collect::<HashSet<_>>();
    // disjoint
    if _1_3.is_disjoint(&_6_10) {
        println!("These sets have no elements in common");
    }

    // union
    println!("union _1_3 and _6_10 {:?}", _1_3.union(&_6_10).collect::<HashSet<_>>());


    // difference
    let _1_10 = (1..=10).collect::<HashSet<_>>();
    println!("difference _1_10 and _1_3 {:?}", _1_10.difference(&_1_3).collect::<HashSet<_>>());


}
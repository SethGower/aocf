extern crate aocf;

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2019))
        .day(Some(2))
        .cookie("yoursessioncookiedatagoeshere")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    println!("{}", input);
}

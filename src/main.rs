pub mod day1;
pub mod day2;

use std::env;
use std::collections::HashMap;

fn main() {
    let table = HashMap::from([
        ("day1".to_string(), Box::new(day1::run as fn())),
        ("day2".to_string(), Box::new(day2::run as fn()))
    ]);

    let args: Vec<String> = env::args().collect();

    table[&args[1]]();
}

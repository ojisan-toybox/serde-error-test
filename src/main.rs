use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let deserialized: Point = serde_json::from_str("{}").unwrap();
    println!("deserialized = {:?}", deserialized);
}
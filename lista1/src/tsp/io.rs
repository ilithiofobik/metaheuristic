use super::geo::Point;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_tsp(filename: &str) -> std::io::Result<()> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut coords = HashMap::new();
    let split = contents.split_whitespace();
    let mut counter = 0;

    let mut x = 0.0;
    let mut y: f64;

    for s in split {
        if counter % 3 != 0 {
            if counter % 3 == 1 {
                x = s.parse().unwrap();
            } else {
                y = s.parse().unwrap();
                coords.insert(counter / 3, Point { x, y });
            }
        }
        counter = counter + 1;
    }

    for (_key, _value) in &coords {
        //println!("{}: {:?}", _key, _value);
    }

    return Ok(());
}

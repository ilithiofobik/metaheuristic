use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, another: &Point) -> f64 {
        let x2 = (self.x - another.x).powf(2.0);
        let y2 = (self.y - another.y).powf(2.0);
        return (x2 + y2).sqrt() 
    }
}

fn read_tsp(filename: &str) -> std::io::Result<()> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut coords = HashMap::new();
    let split = contents.split_whitespace();
    let mut counter = 0;
    let mut x = 0.0;
    let mut y = 0.0;

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

    for (key, value) in &coords {
        //println!("{}: {:?}", key, value);
    }

    return Ok(());
}

fn create_tsp(size: i32) -> std::collections::HashMap<i32, Point> {
    let mut coords = HashMap::new();
    let mut rng = rand::thread_rng();

    for n in 0..size {
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let p = Point { x, y };
        coords.insert(n, p);
    }

    return coords;
}

fn main() -> std::io::Result<()> {
    let coords = create_tsp(100);
    for (key, value) in &coords {
        //println!("{}: {:?}", key, value);
    }

    //let p1 = Point { x: 0.0, y: 0.0 };
    //let p2 = Point { x: 1.0, y: 1.0 };
    //println!("{}", p1.distance(&p2));

    return read_tsp("gr666.tsp");
}

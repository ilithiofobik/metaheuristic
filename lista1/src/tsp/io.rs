use super::geo::Point;
use super::geo::Matrix;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_euclid(filename: &str) -> std::io::Result<Matrix> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut coords = HashMap::new();
    let mut counter = 0;

    let mut lines = contents.lines();
    let mut fragment = lines.next();

    while fragment.unwrap() != "NODE_COORD_SECTION" {
        fragment = lines.next();
    }

    fragment = lines.next();
    
    while fragment.unwrap() != "EOF" {
        let split = fragment.unwrap().split_whitespace();
        let vec = split.collect::<Vec<&str>>();
        let x: f64 = vec[1].parse().unwrap();
        let y: f64 = vec[2].parse().unwrap();

        coords.insert(
            counter,
            Point { x, y }
        );

        counter = counter + 1;
        fragment = lines.next();
    }

    let mut matrix = Matrix::new(counter as usize);

    for x in 0..counter {
        for y in 0..counter {
            let val = coords[&x].distance(&coords[&y]);
            matrix.put(x, y, val);
        }
    }

    return Ok(matrix);
}



#[allow(dead_code)]
pub fn read_full_matrix(filename: &str) -> std::io::Result<Matrix> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut counter = 0;

    let mut lines = contents.lines();
    let mut fragment = lines.next();

    while !fragment.unwrap().starts_with("DIMENSION:") {
        fragment = lines.next();
    }

    let size: usize = fragment.unwrap().trim_start_matches("DIMENSION: ").trim().parse().unwrap();
    let mut matrix = Matrix::new(size);

    while fragment.unwrap() != "EDGE_WEIGHT_SECTION" {
        fragment = lines.next();
    }

    fragment = lines.next();
    
    while fragment.unwrap() != "EOF" {
        let split = fragment.unwrap().split_whitespace();
        let vec = split.collect::<Vec<&str>>();

        for word in vec {
            let val: f64 = word.parse().unwrap();
            matrix.put(counter / size, counter % size, val);
            counter = counter + 1;
        }

        fragment = lines.next();
    }

    return Ok(matrix);
}

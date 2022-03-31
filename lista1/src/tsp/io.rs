use super::geo::Matrix;
use super::geo::Point;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_euclid(contents: &String) -> std::io::Result<Matrix> {
    let mut counter = 0;

    let mut lines = contents.lines();
    let mut fragment = lines.next();

    while !fragment.unwrap().starts_with("DIMENSION") {
        fragment = lines.next();
    }

    let size: usize = fragment
        .unwrap()
        .trim_start_matches("DIMENSION")
        .trim()
        .trim_start_matches(":")
        .trim()
        .parse()
        .unwrap();
    let mut matrix = Matrix::new(size);
    let mut coords = HashMap::new();

    while fragment.unwrap() != "NODE_COORD_SECTION" {
        fragment = lines.next();
    }

    fragment = lines.next();

    while counter < size {
        let split = fragment.unwrap().split_whitespace();
        let vec = split.collect::<Vec<&str>>();
        let xf: f64 = vec[1].parse().unwrap();
        let yf: f64 = vec[2].parse().unwrap();
        let x = xf as i64;
        let y = yf as i64;

        coords.insert(counter, Point { x, y });

        counter = counter + 1;
        fragment = lines.next();
    }

    for i in 0..counter {
        let pi = &coords[&i];
        for j in 0..i {
            let pj = &coords[&j];
            let val = pi.distance(&pj);
            matrix.put(i, j, val);
            matrix.put(j, i, val);
        }
    }

    return Ok(matrix);
}

#[allow(dead_code)]
pub fn read_full_matrix(contents: &String) -> std::io::Result<Matrix> {
    let mut counter = 0;

    let mut lines = contents.lines();
    let mut fragment = lines.next();

    while !fragment.unwrap().starts_with("DIMENSION") {
        fragment = lines.next();
    }

    let size: usize = fragment
        .unwrap()
        .trim_start_matches("DIMENSION")
        .trim()
        .trim_start_matches(":")
        .trim()
        .parse()
        .unwrap();
    let mut matrix = Matrix::new(size);

    while fragment.unwrap() != "EDGE_WEIGHT_SECTION" {
        fragment = lines.next();
    }

    fragment = lines.next();

    while counter < size * size {
        let split = fragment.unwrap().split_whitespace();
        let vec = split.collect::<Vec<&str>>();

        for word in vec {
            let fval: f64 = word.parse().unwrap();
            let val = fval as u64;
            matrix.put(counter / size, counter % size, val);
            counter = counter + 1;
        }

        fragment = lines.next();
    }

    return Ok(matrix);
}

#[allow(dead_code)]
pub fn read_lower_matrix(contents: &String) -> std::io::Result<Matrix> {
    let mut x = 0;
    let mut y = 0;

    let mut lines = contents.lines();
    let mut fragment = lines.next();

    while !fragment.unwrap().starts_with("DIMENSION") {
        fragment = lines.next();
    }

    let size: usize = fragment
        .unwrap()
        .trim_start_matches("DIMENSION")
        .trim()
        .trim_start_matches(":")
        .trim()
        .parse()
        .unwrap();
    let mut matrix = Matrix::new(size);

    while fragment.unwrap() != "EDGE_WEIGHT_SECTION" {
        fragment = lines.next();
    }

    fragment = lines.next();

    while x < size {
        let split = fragment.unwrap().split_whitespace();
        let vec = split.collect::<Vec<&str>>();

        for word in vec {
            let fval: f64 = word.parse().unwrap();
            let val = fval as u64;
            matrix.put(x, y, val);
            matrix.put(y, x, val);

            if x == y {
                y = 0;
                x = x + 1;
            } else {
                y = y + 1;
            }
        }

        fragment = lines.next();
    }

    return Ok(matrix);
}

#[allow(dead_code)]
pub fn read_file(filename: &str) -> std::io::Result<Matrix> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.contains("FULL_MATRIX") {
        return read_full_matrix(&contents);
    } else if contents.contains("EUC_2D") {
        return read_euclid(&contents);
    } else if contents.contains("LOWER_DIAG_ROW") {
        return read_lower_matrix(&contents);
    }

    panic!("Wrong format.");
}

pub fn read_tour(filename: &str) -> std::io::Result<Vec<usize>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut perm = Vec::new();

    let mut lines = contents.lines();
    let mut fragment = lines.next();

    while fragment.unwrap() != "TOUR_SECTION" {
        fragment = lines.next();
    }

    fragment = lines.next();

    let mut minus_one = false;

    while !minus_one {
        let split = fragment.unwrap().split_whitespace();
        let vec = split.collect::<Vec<&str>>();

        for word in vec {
            let fval: f64 = word.parse().unwrap();
            let val = fval as i64;

            if val == -1 {
                minus_one = true;
                break;
            }

            perm.push((val - 1) as usize);
        }

        fragment = lines.next();
    }

    return Ok(perm);
}

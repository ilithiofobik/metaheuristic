use pyo3::prelude::*;
use pyo3::types::PyType;

#[derive(Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[allow(dead_code)]
impl Point {
    pub fn distance(&self, another: &Point) -> u64 {
        let x2 = (self.x - another.x).pow(2) as f64;
        let y2 = (self.y - another.y).pow(2) as f64;
        return (x2 + y2).sqrt().round() as u64;
    }
}


#[pyclass]
#[derive(Clone)]
pub struct Matrix {
    pub n: usize,
    pub matrix: Vec<u64>,
}

#[pymethods]
impl Matrix {
    #[new]
    pub fn new(n: usize) -> Matrix {
        let matrix = vec![0; n * n];
        return Matrix { n, matrix };
    }
    
    #[text_signature = "($self, x, y)"]
    pub fn get(&self, x: usize, y: usize) -> u64 {
        return self.matrix[x * self.n + y];
    }

    #[text_signature = "($self, x, y, value)"]
    pub fn put(&mut self, x: usize, y: usize, value: u64) {
        self.matrix[x * self.n + y] = value;
    }
}

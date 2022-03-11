#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn distance(&self, another: &Point) -> f64 {
        let x2 = (self.x - another.x).powf(2.0);
        let y2 = (self.y - another.y).powf(2.0);
        return (x2 + y2).sqrt();
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub n: usize,
    pub matrix: Vec<f64>,
}


#[allow(dead_code)]
impl Matrix {
    pub fn get(&self, x: usize, y: usize) -> f64 {
        return self.matrix[x * self.n + y];
    }

    pub fn put(&mut self, x: usize, y: usize, value: f64) {
        self.matrix[x * self.n + y] = value;
    }

    pub fn new(n: usize) -> Matrix {
        let matrix = vec![0.0; n * n];
        return Matrix { n, matrix }
    }
}
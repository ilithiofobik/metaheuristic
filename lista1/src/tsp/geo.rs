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

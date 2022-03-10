use super::geo::Point;
use rand::Rng;
use std::collections::HashMap;

pub fn create_tsp(size: i32) -> std::collections::HashMap<i32, Point> {
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

use super::geo::Matrix;

pub fn objective_function(permutation: &Vec<usize>, matrix: &Matrix) -> std::io::Result<u64> {
    let mut cost = 0;

    for i in 1..matrix.n - 1 {
        cost = cost + matrix.get(permutation[i - 1], permutation[i]);
    }

    return Ok(cost);
}
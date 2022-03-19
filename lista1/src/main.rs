mod tsp;
use tsp::alg;
use tsp::io;

fn main() {
    let matrix = io::read_file("a280.tsp");

    match matrix {
        // using ref if another match matrix is needed
        Ok(ref m) => {
            let (best_val, best_perm) = alg::two_opt(&m);
            println!("best value is {}", best_val);
            println!("best perm is {:?}", best_perm);
        }
        Err(ref e) => println!("{}", e),
    }
}

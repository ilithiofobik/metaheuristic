mod tsp;
use tsp::alg;
use tsp::io;

fn main() {
    let matrix = io::read_file("a280.tsp");

    match matrix {
        // using ref if another match matrix is needed
        Ok(ref m) => {
            let (neigh_best_val, neigh_best_perm) = alg::nearest_neighbor(&m);
            let (ext_neigh_best_val, ext_neigh_best_perm) = alg::extended_nearest_neighbor(&m);
            let (opt_best_val, opt_best_perm) = alg::two_opt(&m);
            
            println!("[NEIGH] Best value is {}", neigh_best_val);
            println!("[NEIGH] Best perm is {:?}", neigh_best_perm);

            println!("[EXT-NEIGH] Best value is {}", ext_neigh_best_val);
            println!("[EXT-NEIGH] Best perm is {:?}", ext_neigh_best_perm);

            println!("[2-OPT] Best value is {}", opt_best_val);
            println!("[2-OPT] Best perm is {:?}", opt_best_perm);
        }
        Err(ref e) => println!("{}", e),
    }
}

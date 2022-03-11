mod tsp;
use tsp::io;

fn main() {
    let matrix = io::read_euclid("gr666.tsp");
    let matrix2 = io::read_full_matrix("br17.atsp");

    match matrix {
        Ok(_) => println!("Reading euclid went alright."),
        Err(e) => println!("{}", e),
    }

    match matrix2 {
        Ok(_) => println!("Reading full matrix went alright."),
        Err(e) => println!("{}", e),
    }
}

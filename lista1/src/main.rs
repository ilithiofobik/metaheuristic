mod tsp;
use tsp::io;
use tsp::alg;

use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

fn main() {
    let matrix = io::read_euclid("gr666.tsp");
    let matrix2 = io::read_full_matrix("br17.atsp");

    match matrix {
        Ok(m) => {
            println!("Reading euclid went alright.");

            let mut rng = StepRng::new(2, 13);
            let mut irs = Irs::default();

            let mut input : Vec<usize> = (1..m.n).map(|x| x).collect();
            irs.shuffle(&mut input, &mut rng);

            let cost = alg::objective_function(&input, &m);

            match cost {
                Ok(c) => println!("{}", c),
                Err(e) => println!("{}", e),
            }
        },
        Err(e) => println!("{}", e),
    }

    match matrix2 {
        Ok(_) => println!("Reading full matrix went alright."),
        Err(e) => println!("{}", e),
    }
}

mod tsp;
use tsp::alg;
use tsp::io;

use rand::rngs::mock::StepRng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

fn main() {
    let matrix = io::read_file("a280.tsp");
    let matrix2 = io::read_file("br17.atsp");
    let matrix3 = io::read_file("a280.tsp");

    match matrix {
        Ok(m) => {
            println!("Reading euclid went alright.");

            println!("wynik: {}", m.get(0, 1));

            let mut rng = StepRng::new(2, 13);
            let mut irs = Irs::default();

            let mut input: Vec<usize> = (0..m.n).map(|x| x).collect();
            irs.shuffle(&mut input, &mut rng);

            let cost = alg::objective_function(&input, &m);
            println!("{}", cost);
        }
        Err(e) => println!("{}", e),
    }

    match matrix2 {
        Ok(_) => println!("Reading full matrix went alright."),
        Err(e) => println!("{}", e),
    }

    match matrix3 {
        Ok(m) => {
            let (best_val, best_perm) = alg::two_opt(&m);
            println!("best value is {}", best_val);
            println!("best perm is {:?}", best_perm);
        }
        Err(e) => println!("{}", e),
    }
}

mod tsp;
use tsp::gen;
use tsp::tabu;

fn main() {
    let m = gen::create_atsp(200);
    let (val1, val2) = tabu::tabu_search(&m);
    println!("val1 = {}  val2= {:?}", val1, val2);
}

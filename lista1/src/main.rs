mod tsp;
use tsp::test;
use tsp::gen;
use tsp::alg;

fn main() {
    //test::test_tsplib();
    let m = gen::create_atsp(200);
    let (val1, _) = alg::nearest_neighbor(&m);
    let (val2, _) = alg::extended_nearest_neighbor(&m);
    println!("val1 = {}  val2= {}", val1, val2);
    // test::test_time_optimality(gen::create_tsp, "tsp");
    // test::test_time_optimality(gen::create_euclid, "euclid");
    // test::two_opt_test(gen::create_euclid, "euclid");
    // test::test_k_random(gen::create_tsp, "results/k_random_sym.txt");
    // test::test_k_random(gen::create_atsp, "results/k_random_asym.txt");
    // test::test_k_random(gen::create_euclid, "results/k_random_euc.txt");
}

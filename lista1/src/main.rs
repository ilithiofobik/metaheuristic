mod tsp;
use tsp::test;

fn main() {
    test::test_tsplib();
    //test::test_time_optimality();
    //test::test_k_random(gen::create_tsp, "results/k_random_sym.txt");
    //test::test_k_random(gen::create_atsp, "results/k_random_asym.txt");
    //test::test_k_random(gen::create_euclid, "results/k_random_euc.txt");
}

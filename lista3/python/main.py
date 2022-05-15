from tsp_tabu import *

# m = read_file("python/gr21.tsp")
m = create_euclid(5)
population_alg_no_threads(m, True, 1000, 10, 10, 0, True, True, 0.001, 5.0, 100)
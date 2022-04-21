from tsp_tabu import *

m = read_file("python/gr21.tsp")
(best_val, perm) = tabu_search(m, 13)

print(str(best_val), " ", str(perm))
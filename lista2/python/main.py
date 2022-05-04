from tsp_tabu import *

# m = read_file("python/gr21.tsp")
m = create_euclid(5)
(val, perm) = two_opt(m, True)
(best_val, perm1, _) = tabu_search(m, 13, True, val, perm.copy())
(best_val1, perm2, _) = tabu_search_no_threads(m, 13, True, val, perm.copy())

print(str(best_val), " ", str(perm1))
print(str(best_val1), " ", str(perm2))
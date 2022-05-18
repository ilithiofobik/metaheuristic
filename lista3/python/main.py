from tsp_pop import *

# m = read_file("python/gr21.tsp")
m = create_euclid(8)

(i, j) = two_opt(m, True)
print(i, "\n", j)

x, y = population_alg_no_threads_no_isles(
    m, # matrix
    False, # gen_rand
    1000, # gen_size
    10, # elite_num
    2,  # crossing
    True, # swap change
    2,  # size_of_tournament
    0.001, # mut_chance
    20.0) # max_time

print(x, "\n", y)

a, b = population_alg_no_threads_isles(
    m, # matrix
    False, # gen_rand
    100, # isle_size
    10, # elite_num
    2,  # crossing
    True, # swap change
    2,  # size_of_tournament
    0.001, # mut_chance
    20.0, # max_time
    10, # isles_num
    10) # migration_freq

print(a, "\n", b)
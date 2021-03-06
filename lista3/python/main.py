from tsp_pop import *

# m = read_file("python/gr21.tsp")
m = create_euclid(100)

(i, j) = two_opt(m, True)
print(i, "\n", j)

u, v = population_alg_threads_isles(
    m, # matrix
    True, # gen_rand
    100, # gen_size
    10, # elite_num
    1,  # crossing
    True, # swap change
    0,  # size_of_tournament
    0.001, # mut_chance
    20.0, # max_time
    10,  # isles_num
    10,  # migration_freq
    4 ) # num of threads

print(u, "\n", v)


c, d = population_alg_threads_no_isles(
    m, # matrix
    True, # gen_rand
    100, # gen_size
    10, # elite_num
    1,  # crossing
    True, # swap change
    0,  # size_of_tournament
    0.001, # mut_chance
    20.0, # max_time
    4 ) # num of threads


print(c, "\n", d)

x, y = population_alg_no_threads_no_isles(
    m, # matrix
    True, # gen_rand
    100, # gen_size
    10, # elite_num
    1,  # crossing
    True, # swap change
    0,  # size_of_tournament
    0.001, # mut_chance
    10.0) # max_time

print(x, "\n", y)

a, b = population_alg_no_threads_isles(
    m, # matrix
    True, # gen_rand
    10, # isle_size
    10, # elite_num
    1,  # crossing
    True, # swap change
    0,  # size_of_tournament
    0.001, # mut_chance
    20.0, # max_time
    10, # isles_num
    10) # migration_freq

print(a, "\n", b)

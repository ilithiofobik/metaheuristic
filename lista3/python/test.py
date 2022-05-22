from cgi import test
import csv
from tsp_pop import *
from math import sqrt
from datetime import datetime
import sys
import best_values
import matplotlib.pyplot as plt

MIN_N = 50
STEP = 50
MAX_N = 501
num_of_iter = 10
compared_options = 3

files_euclid = [
        "eil51", "berlin52", "st70", "pr76", "rd100", "lin105", "ch130", "ch150", "tsp225", "a280", "pcb442",
    ]
pre_euclid = "data/euc_2d/"
post_euclid = ".tsp"
name_euclid = "TSPLIB_euclid"

files_asym = [
       "br17", "ftv33", "ft53", "ft70", "kro124", "rbg358", "rbg443"
    ]
pre_asym = "data/asym/"
post_asym = ".atsp"
name_asym = "TSPLIB_asym"


# Domyślne wartości
def_gen_rand = False        # True
def_gen_size = 100          # 50  150
def_elite_num = 10          # 5   15
def_cross_op = 1            # 0   2   3
def_swap_change = True      # False
def_size_of_tournament = 0  # 5   10
def_mut_chance = 0.001      # 0.005   0.010
def_max_time = 10.0         # 5.0     15.0
def_isles_num = 10          # 5   15
def_migration_freq = 10     # 5   15
def_num_of_threads = 4      # 2   8



x = []

prd_def = [[], [], [], []]
prd_tabu = []
# prd_gen_rand_false = [[], [], [], []]
prd_gen_rand_true = [[], [], [], []]
prd_gen_size_50 = [[], [], [], []]
# prd_gen_size_100 = [[], [], [], []]
prd_gen_size_150 = [[], [], [], []]
prd_elite_num_5 = [[], [], [], []]
# prd_elite_num_10 = [[], [], [], []]
prd_elite_num_15 = [[], [], [], []]
prd_cross_op_0 = [[], [], [], []]
# prd_cross_op_1 = [[], [], [], []]
prd_cross_op_2 = [[], [], [], []]
prd_cross_op_3 = [[], [], [], []]
# prd_swap_true = [[], [], [], []]
prd_swap_false = [[], [], [], []]
# prd_tour_0 = [[], [], [], []]
prd_tour_5 = [[], [], [], []]
prd_tour_10 = [[], [], [], []]
# prd_mut_chance_1 = [[], [], [], []]
prd_mut_chance_5 = [[], [], [], []]
prd_mut_chance_10 = [[], [], [], []]
prd_max_time_5 = [[], [], [], []]
# prd_max_time_10 = [[], [], [], []]
prd_max_time_15 = [[], [], [], []]
prd_isles_5 = [[], [], [], []]
# prd_isles_10 = [[], [], [], []]
prd_isles_15 = [[], [], [], []]
prd_migration_freq_5 = [[], [], [], []]
# prd_migration_freq_10 = [[], [], [], []]
prd_migration_freq_15 = [[], [], [], []]
prd_num_of_threads_2 = [[], [], [], []]
# prd_num_of_threads_4 = [[], [], [], []]
prd_num_of_threads_8 = [[], [], [], []]

prd = [prd_def, prd_tabu, prd_gen_rand_true, prd_gen_size_50, prd_gen_size_150, prd_elite_num_5, prd_elite_num_15, prd_cross_op_0, prd_cross_op_2, prd_cross_op_3,
    prd_swap_false, prd_tour_5, prd_tour_10, prd_mut_chance_5, prd_mut_chance_10, prd_max_time_5, prd_max_time_15, prd_isles_5, prd_isles_15, prd_migration_freq_5, prd_migration_freq_15,
    prd_num_of_threads_2, prd_num_of_threads_8]


def test_tsplib(files, pre, post, name, alg_num):
    for file in files:
        m = read_file(pre + file + post)
        min_val = best_values.best_value(file)
        (best_value, best_perm) = two_opt(m, True)
        n = len(best_perm)


        if alg_num == 1:
            print(str(datetime.now()) + ":  " + pre + file + post)

            (val_tabu, _, _) = tabu_search(m, n, True, best_value, best_perm)
            (val_def, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_gen_rand_true, _) = population_alg_no_threads_no_isles(m, True, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_gen_size_50, _) = population_alg_no_threads_no_isles(m, def_gen_rand, 50, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_gen_size_150, _) = population_alg_no_threads_no_isles(m, def_gen_rand, 150, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_elite_num_5, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, 5, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_elite_num_15, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, 15, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_cross_op_0, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, 0, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_cross_op_2, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, 2, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
        
            print(str(datetime.now()) + ":  polowa")

            (val_cross_op_3, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, 3, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_swap_false, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, False, def_size_of_tournament, def_mut_chance, def_max_time)
            (val_tour_5, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 5, def_mut_chance, def_max_time)
            (val_tour_10, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 10, def_mut_chance, def_max_time)
            (val_mut_chance_5, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.005, def_max_time)
            (val_mut_chance_10, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.010, def_max_time)
            (val_max_time_5, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 5)
            (val_max_time_15, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 15)

            n_prd_tabu = ((val_tabu - min_val) / min_val) * 100.0
            n_prd_def = ((val_def - min_val) / min_val) * 100.0
            n_prd_gen_rand_true = ((val_gen_rand_true - min_val) / min_val) * 100.0
            n_prd_gen_size_50 = ((val_gen_size_50 - min_val) / min_val) * 100.0
            n_prd_gen_size_150 = ((val_gen_size_150 - min_val) / min_val) * 100.0
            n_prd_elite_num_5 = ((val_elite_num_5 - min_val) / min_val) * 100.0
            n_prd_elite_num_15 = ((val_elite_num_15 - min_val) / min_val) * 100.0
            n_prd_cross_op_0 = ((val_cross_op_0 - min_val) / min_val) * 100.0
            n_prd_cross_op_2 = ((val_cross_op_2 - min_val) / min_val) * 100.0
            n_prd_cross_op_3 = ((val_cross_op_3 - min_val) / min_val) * 100.0
            n_prd_swap_false = ((val_swap_false - min_val) / min_val) * 100.0
            n_prd_tour_5 = ((val_tour_5 - min_val) / min_val) * 100.0
            n_prd_tour_10 = ((val_tour_10 - min_val) / min_val) * 100.0
            n_prd_mut_chance_5 = ((val_mut_chance_5 - min_val) / min_val) * 100.0
            n_prd_mut_chance_10 = ((val_mut_chance_10 - min_val) / min_val) * 100.0
            n_prd_max_time_5 = ((val_max_time_5 - min_val) / min_val) * 100.0
            n_prd_max_time_15 = ((val_max_time_15 - min_val) / min_val) * 100.0

            f = open("results/1.txt", "a")
            f.write(str(n) + ";" + str(n_prd_tabu) + ";" + str(n_prd_def) + ";" + str(n_prd_gen_rand_true) + ";" + str(n_prd_gen_size_50) + ";" + str(n_prd_gen_size_150)
            + ";" + str(n_prd_elite_num_5) + ";" + str(n_prd_elite_num_15) + ";" + str(n_prd_cross_op_0) + ";" + str(n_prd_cross_op_2)
            + ";" + str(n_prd_cross_op_3) + ";" + str(n_prd_swap_false) + ";" + str(n_prd_tour_5) + ";" + str(n_prd_tour_10) + ";" + str(n_prd_mut_chance_5)
            + ";" + str(n_prd_mut_chance_10) + ";" + str(n_prd_max_time_5) + ";" + str(n_prd_max_time_15) + "\n")
            f.close()

        if alg_num == 2:
            print(str(datetime.now()) + ":  " + pre + file + post)

            (val_tabu, _, _) = tabu_search(m, n, True, best_value, best_perm)
            (val_def, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_gen_rand_true, _) = population_alg_no_threads_isles(m, True, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_gen_size_50, _) = population_alg_no_threads_isles(m, def_gen_rand, 50, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_gen_size_150, _) = population_alg_no_threads_isles(m, def_gen_rand, 150, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_elite_num_5, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, 5, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_elite_num_15, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, 15, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            
            print(str(datetime.now()) + ":  1 / 3")

            (val_cross_op_0, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 0, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_cross_op_2, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 2, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_cross_op_3, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 3, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_swap_false, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, False, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_tour_5, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 5, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_tour_10, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 10, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
            (val_mut_chance_5, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.005, def_max_time, def_isles_num, def_migration_freq)
            
            print(str(datetime.now()) + ":  2 / 3")
            
            (val_mut_chance_10, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.010, def_max_time, def_isles_num, def_migration_freq)
            (val_max_time_5, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 5, def_isles_num, def_migration_freq)
            (val_max_time_15, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 15, def_isles_num, def_migration_freq)
            (val_isles_num_5, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 5, def_migration_freq)
            (val_isles_num_15, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 15, def_migration_freq)
            (val_migration_freq_5, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, 5)
            (val_migration_freq_15, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, 15)

            n_prd_tabu = ((val_tabu - min_val) / min_val) * 100.0
            n_prd_def = ((val_def - min_val) / min_val) * 100.0
            n_prd_gen_rand_true = ((val_gen_rand_true - min_val) / min_val) * 100.0
            n_prd_gen_size_50 = ((val_gen_size_50 - min_val) / min_val) * 100.0
            n_prd_gen_size_150 = ((val_gen_size_150 - min_val) / min_val) * 100.0
            n_prd_elite_num_5 = ((val_elite_num_5 - min_val) / min_val) * 100.0
            n_prd_elite_num_15 = ((val_elite_num_15 - min_val) / min_val) * 100.0
            n_prd_cross_op_0 = ((val_cross_op_0 - min_val) / min_val) * 100.0
            n_prd_cross_op_2 = ((val_cross_op_2 - min_val) / min_val) * 100.0
            n_prd_cross_op_3 = ((val_cross_op_3 - min_val) / min_val) * 100.0
            n_prd_swap_false = ((val_swap_false - min_val) / min_val) * 100.0
            n_prd_tour_5 = ((val_tour_5 - min_val) / min_val) * 100.0
            n_prd_tour_10 = ((val_tour_10 - min_val) / min_val) * 100.0
            n_prd_mut_chance_5 = ((val_mut_chance_5 - min_val) / min_val) * 100.0
            n_prd_mut_chance_10 = ((val_mut_chance_10 - min_val) / min_val) * 100.0
            n_prd_max_time_5 = ((val_max_time_5 - min_val) / min_val) * 100.0
            n_prd_max_time_15 = ((val_max_time_15 - min_val) / min_val) * 100.0
            n_prd_isles_num_5 = ((val_isles_num_5 - min_val) / min_val) * 100.0
            n_prd_isles_num_15 = ((val_isles_num_15 - min_val) / min_val) * 100.0
            n_prd_migration_freq_5 = ((val_migration_freq_5 - min_val) / min_val) * 100.0
            n_prd_migration_freq_15 = ((val_migration_freq_15 - min_val) / min_val) * 100.0

            f = open("results/2.txt", "a")
            f.write(str(n) + ";" + str(n_prd_tabu) + ";" + str(n_prd_def) + ";" + str(n_prd_gen_rand_true) + ";" + str(n_prd_gen_size_50) + ";" + str(n_prd_gen_size_150)
            + ";" + str(n_prd_elite_num_5) + ";" + str(n_prd_elite_num_15) + ";" + str(n_prd_cross_op_0) + ";" + str(n_prd_cross_op_2)
            + ";" + str(n_prd_cross_op_3) + ";" + str(n_prd_swap_false) + ";" + str(n_prd_tour_5) + ";" + str(n_prd_tour_10) + ";" + str(n_prd_mut_chance_5)
            + ";" + str(n_prd_mut_chance_10) + ";" + str(n_prd_max_time_5) + ";" + str(n_prd_max_time_15) + ";" + str(n_prd_isles_num_5) +";" + str(n_prd_isles_num_15)
            + ";" + str(n_prd_migration_freq_5) + ";" + str(n_prd_migration_freq_15) + "\n")
            f.close()

        if alg_num == 3:
            print(str(datetime.now()) + ":  " + pre + file + post)

            (val_tabu, _, _) = tabu_search(m, n, True, best_value, best_perm)
            (val_def, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_gen_rand_true, _) = population_alg_threads_no_isles(m, True, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_gen_size_50, _) = population_alg_threads_no_isles(m, def_gen_rand, 50, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_gen_size_150, _) = population_alg_threads_no_isles(m, def_gen_rand, 150, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_elite_num_5, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, 5, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_elite_num_15, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, 15, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            
            print(str(datetime.now()) + ":  1 / 3")

            (val_cross_op_0, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, 0, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_cross_op_2, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, 2, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_cross_op_3, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, 3, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_swap_false, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, False, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
            (val_tour_5, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 5, def_mut_chance, def_max_time, def_num_of_threads)
            (val_tour_10, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 10, def_mut_chance, def_max_time, def_num_of_threads)
            (val_mut_chance_5, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.005, def_max_time, def_num_of_threads)
            
            print(str(datetime.now()) + ":  2 / 3")
            
            (val_mut_chance_10, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.010, def_max_time, def_num_of_threads)
            (val_max_time_5, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 5, def_num_of_threads)
            (val_max_time_15, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 15, def_num_of_threads)
            (val_num_of_threads_2, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 2)
            (val_num_of_threads_8, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 8)

            n_prd_tabu = ((val_tabu - min_val) / min_val) * 100.0
            n_prd_def = ((val_def - min_val) / min_val) * 100.0
            n_prd_gen_rand_true = ((val_gen_rand_true - min_val) / min_val) * 100.0
            n_prd_gen_size_50 = ((val_gen_size_50 - min_val) / min_val) * 100.0
            n_prd_gen_size_150 = ((val_gen_size_150 - min_val) / min_val) * 100.0
            n_prd_elite_num_5 = ((val_elite_num_5 - min_val) / min_val) * 100.0
            n_prd_elite_num_15 = ((val_elite_num_15 - min_val) / min_val) * 100.0
            n_prd_cross_op_0 = ((val_cross_op_0 - min_val) / min_val) * 100.0
            n_prd_cross_op_2 = ((val_cross_op_2 - min_val) / min_val) * 100.0
            n_prd_cross_op_3 = ((val_cross_op_3 - min_val) / min_val) * 100.0
            n_prd_swap_false = ((val_swap_false - min_val) / min_val) * 100.0
            n_prd_tour_5 = ((val_tour_5 - min_val) / min_val) * 100.0
            n_prd_tour_10 = ((val_tour_10 - min_val) / min_val) * 100.0
            n_prd_mut_chance_5 = ((val_mut_chance_5 - min_val) / min_val) * 100.0
            n_prd_mut_chance_10 = ((val_mut_chance_10 - min_val) / min_val) * 100.0
            n_prd_max_time_5 = ((val_max_time_5 - min_val) / min_val) * 100.0
            n_prd_max_time_15 = ((val_max_time_15 - min_val) / min_val) * 100.0
            n_prd_num_of_threads_2 = ((val_num_of_threads_2 - min_val) / min_val) * 100.0
            n_prd_num_of_threads_8 = ((val_num_of_threads_8 - min_val) / min_val) * 100.0

            f = open("results/3.txt", "a")
            f.write(str(n) + ";" + str(n_prd_tabu) + ";" + str(n_prd_def) + ";" + str(n_prd_gen_rand_true) + ";" + str(n_prd_gen_size_50) + ";" + str(n_prd_gen_size_150)
            + ";" + str(n_prd_elite_num_5) + ";" + str(n_prd_elite_num_15) + ";" + str(n_prd_cross_op_0) + ";" + str(n_prd_cross_op_2)
            + ";" + str(n_prd_cross_op_3) + ";" + str(n_prd_swap_false) + ";" + str(n_prd_tour_5) + ";" + str(n_prd_tour_10) + ";" + str(n_prd_mut_chance_5)
            + ";" + str(n_prd_mut_chance_10) + ";" + str(n_prd_max_time_5) + ";" + str(n_prd_max_time_15) + ";" + str(n_prd_num_of_threads_2) +";" + str(n_prd_num_of_threads_8) + "\n")
            f.close()

        if alg_num == 4:
            print(str(datetime.now()) + ":  " + pre + file + post)

            (val_tabu, _, _) = tabu_search(m, n, True, best_value, best_perm)
            (val_def, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_gen_rand_true, _) = population_alg_threads_isles(m, True, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_gen_size_50, _) = population_alg_threads_isles(m, def_gen_rand, 50, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_gen_size_150, _) = population_alg_threads_isles(m, def_gen_rand, 150, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_elite_num_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, 5, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_elite_num_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, 15, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            
            print(str(datetime.now()) + ":  1 / 3")

            (val_cross_op_0, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 0, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_cross_op_2, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 2, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_cross_op_3, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 3, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_swap_false, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, False, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_tour_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 5, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_tour_10, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 10, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_mut_chance_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.005, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            
            print(str(datetime.now()) + ":  2 / 3")
            
            (val_mut_chance_10, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.010, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_max_time_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 5, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_max_time_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 15, def_isles_num, def_migration_freq, def_num_of_threads)
            (val_isles_num_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 5, def_migration_freq, def_num_of_threads)
            (val_isles_num_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 15, def_migration_freq, def_num_of_threads)
            (val_migration_freq_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, 5, def_num_of_threads)
            (val_migration_freq_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, 15, def_num_of_threads)
            (val_num_of_threads_2, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, 2)
            (val_num_of_threads_8, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, 8)

            n_prd_tabu = ((val_tabu - min_val) / min_val) * 100.0
            n_prd_def = ((val_def - min_val) / min_val) * 100.0
            n_prd_gen_rand_true = ((val_gen_rand_true - min_val) / min_val) * 100.0
            n_prd_gen_size_50 = ((val_gen_size_50 - min_val) / min_val) * 100.0
            n_prd_gen_size_150 = ((val_gen_size_150 - min_val) / min_val) * 100.0
            n_prd_elite_num_5 = ((val_elite_num_5 - min_val) / min_val) * 100.0
            n_prd_elite_num_15 = ((val_elite_num_15 - min_val) / min_val) * 100.0
            n_prd_cross_op_0 = ((val_cross_op_0 - min_val) / min_val) * 100.0
            n_prd_cross_op_2 = ((val_cross_op_2 - min_val) / min_val) * 100.0
            n_prd_cross_op_3 = ((val_cross_op_3 - min_val) / min_val) * 100.0
            n_prd_swap_false = ((val_swap_false - min_val) / min_val) * 100.0
            n_prd_tour_5 = ((val_tour_5 - min_val) / min_val) * 100.0
            n_prd_tour_10 = ((val_tour_10 - min_val) / min_val) * 100.0
            n_prd_mut_chance_5 = ((val_mut_chance_5 - min_val) / min_val) * 100.0
            n_prd_mut_chance_10 = ((val_mut_chance_10 - min_val) / min_val) * 100.0
            n_prd_max_time_5 = ((val_max_time_5 - min_val) / min_val) * 100.0
            n_prd_max_time_15 = ((val_max_time_15 - min_val) / min_val) * 100.0
            n_prd_isles_num_5 = ((val_isles_num_5 - min_val) / min_val) * 100.0
            n_prd_isles_num_15 = ((val_isles_num_15 - min_val) / min_val) * 100.0
            n_prd_migration_freq_5 = ((val_migration_freq_5 - min_val) / min_val) * 100.0
            n_prd_migration_freq_15 = ((val_migration_freq_15 - min_val) / min_val) * 100.0
            n_prd_num_of_threads_2 = ((val_num_of_threads_2 - min_val) / min_val) * 100.0
            n_prd_num_of_threads_8 = ((val_num_of_threads_8 - min_val) / min_val) * 100.0

            f = open("results/4.txt", "a")
            f.write(str(n) + ";" + str(n_prd_tabu) + ";" + str(n_prd_def) + ";" + str(n_prd_gen_rand_true) + ";" + str(n_prd_gen_size_50) + ";" + str(n_prd_gen_size_150)
            + ";" + str(n_prd_elite_num_5) + ";" + str(n_prd_elite_num_15) + ";" + str(n_prd_cross_op_0) + ";" + str(n_prd_cross_op_2)
            + ";" + str(n_prd_cross_op_3) + ";" + str(n_prd_swap_false) + ";" + str(n_prd_tour_5) + ";" + str(n_prd_tour_10) + ";" + str(n_prd_mut_chance_5)
            + ";" + str(n_prd_mut_chance_10) + ";" + str(n_prd_max_time_5) + ";" + str(n_prd_max_time_15) + ";" + str(n_prd_isles_num_5) + ";" + str(n_prd_isles_num_15)
            + ";" + str(n_prd_migration_freq_5) + ";" + str(n_prd_migration_freq_15) + ";" + str(n_prd_num_of_threads_2) +";" + str(n_prd_num_of_threads_8) + "\n")
            f.close()


def gen_plots_tsplib(name):
    plots_1 = csv.reader(open('results/1.txt', 'r'), delimiter=';')
    plots_2 = csv.reader(open('results/2.txt', 'r'), delimiter=';')
    plots_3 = csv.reader(open('results/3.txt', 'r'), delimiter=';')
    plots_4 = csv.reader(open('results/4.txt', 'r'), delimiter=';')

    for row in plots_1:
        i = 0

        x.append(int(row[0]))
        prd_tabu.append(float(row[1]))
        prd_def[i].append(float(row[2]))
        prd_gen_rand_true[i].append(float(row[3]))
        prd_gen_size_50[i].append(float(row[4]))
        prd_gen_size_150[i].append(float(row[5]))
        prd_elite_num_5[i].append(float(row[6]))
        prd_elite_num_15[i].append(float(row[7]))
        prd_cross_op_0[i].append(float(row[8]))
        prd_cross_op_2[i].append(float(row[9]))
        prd_cross_op_3[i].append(float(row[10]))
        prd_swap_false[i].append(float(row[11]))
        prd_tour_5[i].append(float(row[12]))
        prd_tour_10[i].append(float(row[13]))
        prd_mut_chance_5[i].append(float(row[14]))
        prd_mut_chance_10[i].append(float(row[15]))
        prd_max_time_5[i].append(float(row[16]))
        prd_max_time_15[i].append(float(row[17]))

    for row in plots_2:
        i = 1

        prd_def[i].append(float(row[2]))
        prd_gen_rand_true[i].append(float(row[3]))
        prd_gen_size_50[i].append(float(row[4]))
        prd_gen_size_150[i].append(float(row[5]))
        prd_elite_num_5[i].append(float(row[6]))
        prd_elite_num_15[i].append(float(row[7]))
        prd_cross_op_0[i].append(float(row[8]))
        prd_cross_op_2[i].append(float(row[9]))
        prd_cross_op_3[i].append(float(row[10]))
        prd_swap_false[i].append(float(row[11]))
        prd_tour_5[i].append(float(row[12]))
        prd_tour_10[i].append(float(row[13]))
        prd_mut_chance_5[i].append(float(row[14]))
        prd_mut_chance_10[i].append(float(row[15]))
        prd_max_time_5[i].append(float(row[16]))
        prd_max_time_15[i].append(float(row[17]))
        prd_isles_5[i].append(float(row[18]))
        prd_isles_15[i].append(float(row[19]))
        prd_migration_freq_5[i].append(float(row[20]))
        prd_migration_freq_15[i].append(float(row[21]))

    for row in plots_3:
        i = 2

        prd_def[i].append(float(row[2]))
        prd_gen_rand_true[i].append(float(row[3]))
        prd_gen_size_50[i].append(float(row[4]))
        prd_gen_size_150[i].append(float(row[5]))
        prd_elite_num_5[i].append(float(row[6]))
        prd_elite_num_15[i].append(float(row[7]))
        prd_cross_op_0[i].append(float(row[8]))
        prd_cross_op_2[i].append(float(row[9]))
        prd_cross_op_3[i].append(float(row[10]))
        prd_swap_false[i].append(float(row[11]))
        prd_tour_5[i].append(float(row[12]))
        prd_tour_10[i].append(float(row[13]))
        prd_mut_chance_5[i].append(float(row[14]))
        prd_mut_chance_10[i].append(float(row[15]))
        prd_max_time_5[i].append(float(row[16]))
        prd_max_time_15[i].append(float(row[17]))
        prd_num_of_threads_2[i].append(float(row[18]))
        prd_num_of_threads_8[i].append(float(row[19]))

    for row in plots_4:
        i = 3

        prd_def[i].append(float(row[2]))
        prd_gen_rand_true[i].append(float(row[3]))
        prd_gen_size_50[i].append(float(row[4]))
        prd_gen_size_150[i].append(float(row[5]))
        prd_elite_num_5[i].append(float(row[6]))
        prd_elite_num_15[i].append(float(row[7]))
        prd_cross_op_0[i].append(float(row[8]))
        prd_cross_op_2[i].append(float(row[9]))
        prd_cross_op_3[i].append(float(row[10]))
        prd_swap_false[i].append(float(row[11]))
        prd_tour_5[i].append(float(row[12]))
        prd_tour_10[i].append(float(row[13]))
        prd_mut_chance_5[i].append(float(row[14]))
        prd_mut_chance_10[i].append(float(row[15]))
        prd_max_time_5[i].append(float(row[16]))
        prd_max_time_15[i].append(float(row[17]))
        prd_isles_5[i].append(float(row[18]))
        prd_isles_15[i].append(float(row[19]))
        prd_migration_freq_5[i].append(float(row[20]))
        prd_migration_freq_15[i].append(float(row[21]))
        prd_num_of_threads_2[i].append(float(row[22]))
        prd_num_of_threads_8[i].append(float(row[23]))


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Comparison with Tabu')
    plt.plot(x, prd_tabu, label='tabu search')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr')
    plt.plot(x, prd_def[1], label='Isl No-Thr')
    plt.plot(x, prd_def[2], label='No-Isl Thr')
    plt.plot(x, prd_def[3], label='Isl Thr')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Random start')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr 2-OPT')
    plt.plot(x, prd_def[1], label='Isl No-Thr 2-OPT')
    plt.plot(x, prd_def[2], label='No-Isl Thr 2-OPT')
    plt.plot(x, prd_def[3], label='Isl Thr 2-OPT')
    plt.plot(x, prd_gen_rand_true[0], label='No-Isl No-Thr random')
    plt.plot(x, prd_gen_rand_true[1], label='Isl No-Thr random')
    plt.plot(x, prd_gen_rand_true[2], label='No-Isl Thr random')
    plt.plot(x, prd_gen_rand_true[3], label='Isl Thr random')
    plt.legend()
    plt.savefig('results/plots/gen_rand_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Generation size')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr 100')
    plt.plot(x, prd_def[1], label='Isl No-Thr 100')
    plt.plot(x, prd_def[2], label='No-Isl Thr 100')
    plt.plot(x, prd_def[3], label='Isl Thr 100')
    plt.plot(x, prd_gen_size_50[0], label='No-Isl No-Thr 50')
    plt.plot(x, prd_gen_size_50[1], label='Isl No-Thr 50')
    plt.plot(x, prd_gen_size_50[2], label='No-Isl Thr 50')
    plt.plot(x, prd_gen_size_50[3], label='Isl Thr 50')
    plt.plot(x, prd_gen_size_150[0], label='No-Isl No-Thr 150')
    plt.plot(x, prd_gen_size_150[1], label='Isl No-Thr 150')
    plt.plot(x, prd_gen_size_150[2], label='No-Isl Thr 150')
    plt.plot(x, prd_gen_size_150[3], label='Isl Thr 150')
    plt.legend()
    plt.savefig('results/plots/gen_size_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Elite size')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr 10')
    plt.plot(x, prd_def[1], label='Isl No-Thr 10')
    plt.plot(x, prd_def[2], label='No-Isl Thr 10')
    plt.plot(x, prd_def[3], label='Isl Thr 10')
    plt.plot(x, prd_elite_num_5[0], label='No-Isl No-Thr 5')
    plt.plot(x, prd_elite_num_5[1], label='Isl No-Thr 5')
    plt.plot(x, prd_elite_num_5[2], label='No-Isl Thr 5')
    plt.plot(x, prd_elite_num_5[3], label='Isl Thr 5')
    plt.plot(x, prd_elite_num_15[0], label='No-Isl No-Thr 15')
    plt.plot(x, prd_elite_num_15[1], label='Isl No-Thr 15')
    plt.plot(x, prd_elite_num_15[2], label='No-Isl Thr 15')
    plt.plot(x, prd_elite_num_15[3], label='Isl Thr 15')
    plt.legend()
    plt.savefig('results/plots/elite_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Crossing')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr 1')
    plt.plot(x, prd_def[1], label='Isl No-Thr 1')
    plt.plot(x, prd_def[2], label='No-Isl Thr 1')
    plt.plot(x, prd_def[3], label='Isl Thr 1')
    plt.plot(x, prd_cross_op_0[0], label='No-Isl No-Thr 0')
    plt.plot(x, prd_cross_op_0[1], label='Isl No-Thr 0')
    plt.plot(x, prd_cross_op_0[2], label='No-Isl Thr 0')
    plt.plot(x, prd_cross_op_0[3], label='Isl Thr 0')
    plt.plot(x, prd_cross_op_2[0], label='No-Isl No-Thr 2')
    plt.plot(x, prd_cross_op_2[1], label='Isl No-Thr 2')
    plt.plot(x, prd_cross_op_2[2], label='No-Isl Thr 2')
    plt.plot(x, prd_cross_op_2[3], label='Isl Thr 2')
    plt.plot(x, prd_cross_op_3[0], label='No-Isl No-Thr 3')
    plt.plot(x, prd_cross_op_3[1], label='Isl No-Thr 3')
    plt.plot(x, prd_cross_op_3[2], label='No-Isl Thr 3')
    plt.plot(x, prd_cross_op_3[3], label='Isl Thr 3')
    plt.legend()
    plt.savefig('results/plots/crossing_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Swap')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr true')
    plt.plot(x, prd_def[1], label='Isl No-Thr true')
    plt.plot(x, prd_def[2], label='No-Isl Thr true')
    plt.plot(x, prd_def[3], label='Isl Thr true')
    plt.plot(x, prd_swap_false[0], label='No-Isl No-Thr false')
    plt.plot(x, prd_swap_false[1], label='Isl No-Thr false')
    plt.plot(x, prd_swap_false[2], label='No-Isl Thr false')
    plt.plot(x, prd_swap_false[3], label='Isl Thr false')
    plt.legend()
    plt.savefig('results/plots/swap_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Tour')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr 0')
    plt.plot(x, prd_def[1], label='Isl No-Thr 0')
    plt.plot(x, prd_def[2], label='No-Isl Thr 0')
    plt.plot(x, prd_def[3], label='Isl Thr 0')
    plt.plot(x, prd_tour_5[0], label='No-Isl No-Thr 5')
    plt.plot(x, prd_tour_5[1], label='Isl No-Thr 5')
    plt.plot(x, prd_tour_5[2], label='No-Isl Thr 5')
    plt.plot(x, prd_tour_5[3], label='Isl Thr 5')
    plt.plot(x, prd_tour_10[0], label='No-Isl No-Thr 10')
    plt.plot(x, prd_tour_10[1], label='Isl No-Thr 10')
    plt.plot(x, prd_tour_10[2], label='No-Isl Thr 10')
    plt.plot(x, prd_tour_10[3], label='Isl Thr 10')
    plt.legend()
    plt.savefig('results/plots/tour_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Mut chance')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr 0.001')
    plt.plot(x, prd_def[1], label='Isl No-Thr 0.001')
    plt.plot(x, prd_def[2], label='No-Isl Thr 0.001')
    plt.plot(x, prd_def[3], label='Isl Thr 0.001')
    plt.plot(x, prd_mut_chance_5[0], label='No-Isl No-Thr 0.005')
    plt.plot(x, prd_mut_chance_5[1], label='Isl No-Thr 0.005')
    plt.plot(x, prd_mut_chance_5[2], label='No-Isl Thr 0.005')
    plt.plot(x, prd_mut_chance_5[3], label='Isl Thr 0.005')
    plt.plot(x, prd_mut_chance_10[0], label='No-Isl No-Thr 0.010')
    plt.plot(x, prd_mut_chance_10[1], label='Isl No-Thr 0.010')
    plt.plot(x, prd_mut_chance_10[2], label='No-Isl Thr 0.010')
    plt.plot(x, prd_mut_chance_10[3], label='Isl Thr 0.010')
    plt.legend()
    plt.savefig('results/plots/mut_chance_' + name.lower() + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Max time')
    plt.plot(x, prd_def[0], label='No-Isl No-Thr 10')
    plt.plot(x, prd_def[1], label='Isl No-Thr 10')
    plt.plot(x, prd_def[2], label='No-Isl Thr 10')
    plt.plot(x, prd_def[3], label='Isl Thr 10')
    plt.plot(x, prd_max_time_5[0], label='No-Isl No-Thr 5')
    plt.plot(x, prd_max_time_5[1], label='Isl No-Thr 5')
    plt.plot(x, prd_max_time_5[2], label='No-Isl Thr 5')
    plt.plot(x, prd_max_time_5[3], label='Isl Thr 5')
    plt.plot(x, prd_max_time_15[0], label='No-Isl No-Thr 15')
    plt.plot(x, prd_max_time_15[1], label='Isl No-Thr 15')
    plt.plot(x, prd_max_time_15[2], label='No-Isl Thr 15')
    plt.plot(x, prd_max_time_15[3], label='Isl Thr 15')
    plt.legend()
    plt.savefig('results/plots/max_time_' + name.lower() + '.png')


    
# test_tsplib(files_euclid, pre_euclid, post_euclid, name_euclid, 1)
# test_tsplib(files_asym, pre_asym, post_asym, name_asym, 1)

gen_plots_tsplib(name_euclid)
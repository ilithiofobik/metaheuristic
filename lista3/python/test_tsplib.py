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
def_mut_chance = 0.050      # 0.001   0.010    0.100
def_max_time = 10.0         # 5.0     15.0
def_isles_num = 10          # 5   15
def_migration_freq = 10     # 5   15
def_num_of_threads = 4      # 2   8


def test_tsplib(files, pre, post, name):
    num_of_files = len(files)
    curr_file = 0

    for file in files:
        curr_file += 1
        m = read_file(pre + file + post)
        min_val = best_values.best_value(file)
        (best_value, best_perm) = two_opt(m, True)
        n = len(best_perm)

        print(str(datetime.now()) + ":    [" + str(curr_file) + "/" + str(num_of_files) + "]  " + pre + file + post + "   0 / 3")

        (val_tabu, _, _) = tabu_search(m, n, True, best_value, best_perm)
        (val_alg_1, _) = population_alg_no_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time)
        (val_alg_2, _) = population_alg_no_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq)
        (val_alg_3, _) = population_alg_threads_no_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_num_of_threads)
        (val_def, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_gen_rand_true, _) = population_alg_threads_isles(m, True, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_gen_size_50, _) = population_alg_threads_isles(m, def_gen_rand, 50, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_gen_size_150, _) = population_alg_threads_isles(m, def_gen_rand, 150, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_elite_num_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, 5, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        
        print(str(datetime.now()) + ":    [" + str(curr_file) + "/" + str(num_of_files) + "]  " + pre + file + post + "   1 / 3")

        (val_elite_num_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, 15, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_cross_op_0, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 0, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_cross_op_2, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 2, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_cross_op_3, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, 3, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_swap_false, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, False, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_tour_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 5, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_tour_10, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, 10, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_mut_chance_0, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.001, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)            
        (val_mut_chance_1, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.010, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)

        print(str(datetime.now()) + ":    [" + str(curr_file) + "/" + str(num_of_files) + "]  " + pre + file + post + "   2 / 3")

        (val_mut_chance_2, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, 0.100, def_max_time, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_max_time_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 5, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_max_time_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, 15, def_isles_num, def_migration_freq, def_num_of_threads)
        (val_isles_num_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 5, def_migration_freq, def_num_of_threads)
        (val_isles_num_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, 15, def_migration_freq, def_num_of_threads)
        (val_migration_freq_5, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, 5, def_num_of_threads)
        (val_migration_freq_15, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, 15, def_num_of_threads)
        (val_num_of_threads_2, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, 2)
        (val_num_of_threads_8, _) = population_alg_threads_isles(m, def_gen_rand, def_gen_size, def_elite_num, def_cross_op, def_swap_change, def_size_of_tournament, def_mut_chance, def_max_time, def_isles_num, def_migration_freq, 8)

        print(str(datetime.now()) + ":    [" + str(curr_file) + "/" + str(num_of_files) + "]  " + pre + file + post + "   3 / 3")

        n_prd_tabu = ((val_tabu - min_val) / min_val) * 100.0
        n_prd_alg_1 = ((val_alg_1 - min_val) / min_val) * 100.0
        n_prd_alg_2 = ((val_alg_2 - min_val) / min_val) * 100.0
        n_prd_alg_3 = ((val_alg_3 - min_val) / min_val) * 100.0
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
        n_prd_mut_chance_0 = ((val_mut_chance_0 - min_val) / min_val) * 100.0
        n_prd_mut_chance_1 = ((val_mut_chance_1 - min_val) / min_val) * 100.0
        n_prd_mut_chance_2 = ((val_mut_chance_2 - min_val) / min_val) * 100.0
        n_prd_max_time_5 = ((val_max_time_5 - min_val) / min_val) * 100.0
        n_prd_max_time_15 = ((val_max_time_15 - min_val) / min_val) * 100.0
        n_prd_isles_num_5 = ((val_isles_num_5 - min_val) / min_val) * 100.0
        n_prd_isles_num_15 = ((val_isles_num_15 - min_val) / min_val) * 100.0
        n_prd_migration_freq_5 = ((val_migration_freq_5 - min_val) / min_val) * 100.0
        n_prd_migration_freq_15 = ((val_migration_freq_15 - min_val) / min_val) * 100.0
        n_prd_num_of_threads_2 = ((val_num_of_threads_2 - min_val) / min_val) * 100.0
        n_prd_num_of_threads_8 = ((val_num_of_threads_8 - min_val) / min_val) * 100.0

        f = open("results/" + name.lower() + ".txt", "a")
        f.write(str(n) + ";" + str(n_prd_tabu) + ";" + str(n_prd_alg_1) + ";" + str(n_prd_alg_2) + ";" + str(n_prd_alg_3) + ";" + str(n_prd_def)
        + ";" + str(n_prd_gen_rand_true) + ";" + str(n_prd_gen_size_50) + ";" + str(n_prd_gen_size_150)  + ";" + str(n_prd_elite_num_5)
        + ";" + str(n_prd_elite_num_15) + ";" + str(n_prd_cross_op_0) + ";" + str(n_prd_cross_op_2) + ";" + str(n_prd_cross_op_3)
        + ";" + str(n_prd_swap_false) + ";" + str(n_prd_tour_5) + ";" + str(n_prd_tour_10) + ";" + str(n_prd_mut_chance_0) + ";" + str(n_prd_mut_chance_1) 
        + ";" + str(n_prd_mut_chance_2) + ";" + str(n_prd_max_time_5) + ";" + str(n_prd_max_time_15) + ";" + str(n_prd_isles_num_5) + ";" + str(n_prd_isles_num_15)
        + ";" + str(n_prd_migration_freq_5) + ";" + str(n_prd_migration_freq_15) + ";" + str(n_prd_num_of_threads_2) +";" + str(n_prd_num_of_threads_8) + "\n")
        f.close()


def gen_plots_tsplib(name):
    x = []

    prd_def = []
    prd_tabu = []
    prd_alg_1 = []
    prd_alg_2 = []
    prd_alg_3 = []
    prd_gen_rand_true = []
    prd_gen_size_50 = []
    prd_gen_size_150 = []
    prd_elite_num_5 = []
    prd_elite_num_15 = []
    prd_cross_op_0 = []
    prd_cross_op_2 = []
    prd_cross_op_3 = []
    prd_swap_false = []
    prd_tour_5 = []
    prd_tour_10 = []
    prd_mut_chance_0 = []
    prd_mut_chance_1 = []
    prd_mut_chance_2 = []
    prd_max_time_5 = []
    prd_max_time_15 = []
    prd_isles_5 = []
    prd_isles_15 = []
    prd_migration_freq_5 = []
    prd_migration_freq_15 = []
    prd_num_of_threads_2 = []
    prd_num_of_threads_8 = []

    plots = csv.reader(open('results/' + name.lower() + '.txt', 'r'), delimiter=';')

    for row in plots:
        x.append(int(row[0]))

        prd_tabu.append(float(row[1]))
        prd_alg_1.append(float(row[2]))
        prd_alg_2.append(float(row[3]))
        prd_alg_3.append(float(row[4]))
        prd_def.append(float(row[5]))
        prd_gen_rand_true.append(float(row[6]))
        prd_gen_size_50.append(float(row[7]))
        prd_gen_size_150.append(float(row[8]))
        prd_elite_num_5.append(float(row[9]))
        prd_elite_num_15.append(float(row[10]))
        prd_cross_op_0.append(float(row[11]))
        prd_cross_op_2.append(float(row[12]))
        prd_cross_op_3.append(float(row[13]))
        prd_swap_false.append(float(row[14]))
        prd_tour_5.append(float(row[15]))
        prd_tour_10.append(float(row[16]))
        prd_mut_chance_0.append(float(row[17]))
        prd_mut_chance_1.append(float(row[18]))
        prd_mut_chance_2.append(float(row[19]))
        prd_max_time_5.append(float(row[20]))
        prd_max_time_15.append(float(row[21]))
        prd_isles_5.append(float(row[22]))
        prd_isles_15.append(float(row[23]))
        prd_migration_freq_5.append(float(row[24]))
        prd_migration_freq_15.append(float(row[25]))
        prd_num_of_threads_2.append(float(row[26]))
        prd_num_of_threads_8.append(float(row[27]))


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Comparison of algorithms')
    plt.plot(x, prd_tabu, label='Tabu search')
    plt.plot(x, prd_alg_1, label='No isles, no threads')
    plt.plot(x, prd_alg_2, label='Isles, no threads')
    plt.plot(x, prd_alg_3, label='No isles, threads')
    plt.plot(x, prd_def, label='Isles, threads [default]')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_1_comparison' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Initial permutation')
    plt.plot(x, prd_def, label='2-OPT')
    plt.plot(x, prd_gen_rand_true, label='Random')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_2_gen_rand' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Generation size')
    plt.plot(x, prd_gen_size_50, label='50')
    plt.plot(x, prd_def, label='100')
    plt.plot(x, prd_gen_size_150, label='150')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_3_generation_size' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Elite size')
    plt.plot(x, prd_elite_num_5, label='5')
    plt.plot(x, prd_def, label='10')
    plt.plot(x, prd_elite_num_15, label='15')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_4_elite_num' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Crossing methods')
    plt.plot(x, prd_def, label='OX')
    plt.plot(x, prd_cross_op_0, label='HX')
    plt.plot(x, prd_cross_op_2, label='CX')
    plt.plot(x, prd_cross_op_3, label='PMX')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_5_crossing' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Swap and inverse in mutation')
    plt.plot(x, prd_def, label='Swap')
    plt.plot(x, prd_swap_false, label='Inverse')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_6_swap_inverse' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Size of tournament')
    plt.plot(x, prd_def, label='Roulette')
    plt.plot(x, prd_tour_5, label='5')
    plt.plot(x, prd_tour_10, label='10')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_7_tour' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Mutation chance')
    plt.plot(x, prd_mut_chance_0, label='0.001')
    plt.plot(x, prd_mut_chance_1, label='0.010')
    plt.plot(x, prd_def, label='0.050')
    plt.plot(x, prd_mut_chance_2, label='0.100')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_8_mut_chance' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Max time')
    plt.plot(x, prd_max_time_5, label='5 s')
    plt.plot(x, prd_def, label='10 s')
    plt.plot(x, prd_max_time_15, label='15 s')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_9_max_time' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Number of isles')
    plt.plot(x, prd_isles_5, label='5')
    plt.plot(x, prd_def, label='10')
    plt.plot(x, prd_isles_15, label='15')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_10_isles' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Migration frequency')
    plt.plot(x, prd_migration_freq_5, label='5')
    plt.plot(x, prd_def, label='10')
    plt.plot(x, prd_migration_freq_15, label='15')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_11_migration_freq' + '.png')


    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Number of threads')
    plt.plot(x, prd_num_of_threads_2, label='2')
    plt.plot(x, prd_def, label='4')
    plt.plot(x, prd_num_of_threads_8, label='8')
    plt.legend()
    plt.savefig('results/plots/' + name.lower() + '_12_num_of_threads' + '.png')

    
# test_tsplib(files_euclid, pre_euclid, post_euclid, name_euclid)
# test_tsplib(files_asym, pre_asym, post_asym, name_asym)

gen_plots_tsplib(name_euclid)
gen_plots_tsplib(name_asym)
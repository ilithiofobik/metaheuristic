from cgi import test
from tsp_tabu import *
from datetime import datetime
from math import sqrt
from best_values import best_value
import matplotlib.pyplot as plt


num_of_iter = 3
compared_options = 3

files_euclid = [
        "eil51", "berlin52", "st70", "pr76", "rd100", "lin105", "ch130", "ch150", "tsp225", "a280", "pcb442", "pr1002",
    ]
pre_euclid = "data\euc_2d"
post_euclid = ".tsp"

def test_tsplib(files, pre, post):
    pass

# jeden test na wszystko, bo musimy miec optimal do prd
# czas zmieniony na rustowy, ten pythonowy nie jest dokładny
# TODO: można w sumie zrobić tak że najpierw odpalamy raz 2-opta i dajemy permutację 
# jako argument żeby nie liczyć 2-opta wiele razy - wtedy testy szybciej przejdą
def test_tabu(gen, name):
    x = []

    y_prd_13 = []
    y_prd_sqrt = []
    y_prd_default = []
    y_prd_no_threads = []

    y_time_13 = []
    y_time_sqrt = []
    y_time_default = []
    y_time_no_threads = []

    for n in range(50, 350, 50):
        time_13 = 0
        time_sqrt = 0
        time_default = 0
        time_no_threads = 0

        prd_13 = 0
        prd_sqrt = 0
        prd_default = 0
        prd_no_threads = 0

        print(n)

        for _ in range(num_of_iter):
            m = gen(n)
            (best_value, best_perm) = two_opt(m, True)

            (val_13, _, time_13) = tabu_search(m, 13, True, best_value, best_perm)
            (val_sqrt, _, time_sqrt) = tabu_search(m, int(sqrt(n)), True, best_value, best_perm)
            (val_default, _, time_default) = tabu_search(m, n, True, best_value, best_perm)
            (val_no_threads, _, time_no_threads) = tabu_search_no_threads(m, n, True, best_value, best_perm)

            min_val = min(val_13, val_sqrt, val_default, val_no_threads)
            print(f"13={val_13}, sqrt={val_sqrt}, def={val_default}, no_thr={val_no_threads}")
            prd_13 += ((val_13 - min_val) / min_val) * 100.0
            prd_sqrt += ((val_sqrt - min_val) / min_val) * 100.0
            prd_default += ((val_default - min_val) / min_val) * 100.0
            prd_no_threads += ((val_no_threads - min_val) / min_val) * 100.0

        x.append(n)

        y_prd_13.append(prd_13 / num_of_iter)
        y_prd_sqrt.append(prd_sqrt / num_of_iter)
        y_prd_default.append(prd_default / num_of_iter)
        y_prd_no_threads.append(prd_no_threads / num_of_iter)

        y_time_13.append(time_13 / num_of_iter)
        y_time_sqrt.append(time_sqrt / num_of_iter)
        y_time_default.append(time_default / num_of_iter)
        y_time_no_threads.append(time_no_threads / num_of_iter)

    
    # Plots

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Tabu List Length / PRD')
    plt.plot(x, y_prd_13, label='13 elements')
    plt.plot(x, y_prd_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_prd_default, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_time.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Tabu List Length / TIME')
    plt.plot(x, y_time_13, label='13 elements')
    plt.plot(x, y_time_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_time_default, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Multithreading / PRD')
    plt.plot(x, y_prd_default, label='multiple threads')
    plt.plot(x, y_prd_no_threads, label='single thread')
    plt.legend()
    plt.savefig('results/plots/multithreading_' + name.lower() + '_time.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Multithreading / TIME')
    plt.plot(x, y_time_default, label='multiple threads')
    plt.plot(x, y_time_no_threads, label='single thread')
    plt.legend()
    plt.savefig('results/plots/multithreading_' + name.lower() + '_prd.png')

    # Latex tables

    cols = "|" + (compared_options + 1) * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_prd_13[k]) + " & " + str(y_prd_sqrt[k]) + " & " + str(y_prd_default[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"

    with open('results/tables/tabu_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


    cols = "|" + (compared_options + 1) * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_time_13[k]) + " & " + str(y_time_sqrt[k]) + " & " + str(y_time_default[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"
    
    with open('results/tables/tabu_' +  name.lower() + '_time.txt', 'w') as f:
        f.write(text)

    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_prd_default[k]) + " & " + str(y_prd_no_threads[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"

    with open('results/tables/multithreading' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_time_default[k]) + " & " + str(y_time_no_threads[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"
    
    with open('results/tables/multithreading' +  name.lower() + '_time.txt', 'w') as f:
        f.write(text)



test_tabu(create_atsp, 'Asymmetric')
test_tabu(create_euclid, 'Euclid')
from cgi import test
from tsp_tabu import *
from math import sqrt
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

def test_tsplib(files, pre, post, name):
    x = []

    y_prd_13 = []
    y_prd_sqrt = []
    y_prd_default = []
    y_prd_no_threads = []
    y_prd_invert = []
    y_prd_two_opt = []

    y_time_13 = []
    y_time_sqrt = []
    y_time_default = []
    y_time_no_threads = []
    y_time_invert = []

    for file in files:
        m = read_file(pre + file + post)
        min_val = best_values.best_value(file)
        (best_value, best_perm) = two_opt(m, True)
        n = len(best_perm)

        (val_13, _, time_13_a) = tabu_search(m, 13, True, best_value, best_perm)
        (val_sqrt, _, time_sqrt_a) = tabu_search(m, int(sqrt(n)), True, best_value, best_perm)
        (val_default, _, time_default_a) = tabu_search(m, n, True, best_value, best_perm)
        (val_no_threads, _, time_no_threads_a) = tabu_search_no_threads(m, n, True, best_value, best_perm)
        (val_invert, _, time_invert_a) = tabu_search(m, 13, False, best_value, best_perm)

        prd_13 = ((val_13 - min_val) / min_val) * 100.0
        prd_sqrt = ((val_sqrt - min_val) / min_val) * 100.0
        prd_default = ((val_default - min_val) / min_val) * 100.0
        prd_no_threads = ((val_no_threads - min_val) / min_val) * 100.0
        prd_invert = ((val_invert - min_val) / min_val) * 100.0
        prd_two_opt = ((best_value - min_val) / min_val) * 100.0

        time_13 = time_13_a
        time_default = time_default_a
        time_sqrt = time_sqrt_a
        time_no_threads = time_no_threads_a
        time_invert = time_invert_a

        x.append(n)

        y_prd_13.append(prd_13)
        y_prd_sqrt.append(prd_sqrt)
        y_prd_default.append(prd_default)
        y_prd_no_threads.append(prd_no_threads)
        y_prd_invert.append(prd_invert)
        y_prd_two_opt.append(prd_two_opt)

        y_time_13.append(time_13)
        y_time_sqrt.append(time_sqrt)
        y_time_default.append(time_default)
        y_time_no_threads.append(time_no_threads)
        y_time_invert.append(time_invert)

    
    # Plots

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Comparison with 2-OPT / PRD')
    plt.plot(x, y_prd_two_opt, label='2-OPT')
    plt.plot(x, y_prd_default, label='tabu search')
    plt.legend()
    plt.savefig('results/plots/two_opt_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Tabu List Length / PRD')
    plt.plot(x, y_prd_13, label='13 elements')
    plt.plot(x, y_prd_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_prd_default, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Tabu List Length / TIME')
    plt.plot(x, y_time_13, label='13 elements')
    plt.plot(x, y_time_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_time_default, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_time.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Multithreading / PRD')
    plt.plot(x, y_prd_default, label='multiple threads')
    plt.plot(x, y_prd_no_threads, label='single thread')
    plt.legend()
    plt.savefig('results/plots/multithreading_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Multithreading / TIME')
    plt.plot(x, y_time_default, label='multiple threads')
    plt.plot(x, y_time_no_threads, label='single thread')
    plt.legend()
    plt.savefig('results/plots/multithreading_' + name.lower() + '_time.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Neighbourhood / PRD')
    plt.plot(x, y_prd_default, label='swap')
    plt.plot(x, y_prd_invert, label='invert')
    plt.legend()
    plt.savefig('results/plots/neighbours_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Neighbourhood / TIME')
    plt.plot(x, y_time_default, label='swap')
    plt.plot(x, y_time_invert, label='invert')
    plt.legend()
    plt.savefig('results/plots/neighbours_' + name.lower() + '_time.png')

    # Latex tables

    cols = "|" + (3) * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_prd_default[k]) + " & " + str(y_prd_default[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"

    with open('results/tables/two_opt_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


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

    with open('results/tables/multithreading_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_time_default[k]) + " & " + str(y_time_no_threads[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"
    
    with open('results/tables/multithreading_' +  name.lower() + '_time.txt', 'w') as f:
        f.write(text)

    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_prd_default[k]) + " & " + str(y_prd_invert[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"

    with open('results/tables/neighbour_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_time_default[k]) + " & " + str(y_time_invert[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"
    
    with open('results/tables/neighbour_' +  name.lower() + '_time.txt', 'w') as f:
        f.write(text)

    

def test_tabu(gen, name):
    x = []

    y_prd_13 = []
    y_prd_sqrt = []
    y_prd_default = []
    y_prd_no_threads = []
    y_prd_invert = []
    y_prd_two_opt = []

    y_time_13 = []
    y_time_sqrt = []
    y_time_default = []
    y_time_no_threads = []
    y_time_invert = []

    for n in range(MIN_N, MAX_N, STEP):
        time_13 = 0
        time_sqrt = 0
        time_default = 0
        time_no_threads = 0
        time_invert = 0

        prd_13 = 0
        prd_sqrt = 0
        prd_default = 0
        prd_no_threads = 0
        prd_invert = 0
        prd_two_opt = 0

        print(n)

        for _ in range(num_of_iter):
            m = gen(n)
            (best_value, best_perm) = two_opt(m, True)

            (val_13, _, time_13_a) = tabu_search(m, 13, True, best_value, best_perm.copy())
            (val_sqrt, _, time_sqrt_a) = tabu_search(m, int(sqrt(n)), True, best_value, best_perm.copy())
            (val_default, _, time_default_a) = tabu_search(m, n, True, best_value, best_perm.copy())
            (val_no_threads, _, time_no_threads_a) = tabu_search_no_threads(m, n, True, best_value, best_perm.copy())
            (val_invert, _, time_invert_a) = tabu_search(m, 13, False, best_value, best_perm.copy())

            min_val = max(min(val_13, val_sqrt, val_default, val_no_threads, val_invert), 1)
            prd_13 += ((val_13 - min_val) / min_val) * 100.0
            prd_sqrt += ((val_sqrt - min_val) / min_val) * 100.0
            prd_default += ((val_default - min_val) / min_val) * 100.0
            prd_no_threads += ((val_no_threads - min_val) / min_val) * 100.0
            prd_invert += ((val_invert - min_val) / min_val) * 100.0
            prd_two_opt += ((best_value - min_val) / min_val) * 100.0

            time_13 += time_13_a
            time_default += time_default_a
            time_sqrt += time_sqrt_a
            time_no_threads += time_no_threads_a
            time_invert += time_invert_a

        x.append(n)

        y_prd_13.append(prd_13 / num_of_iter)
        y_prd_sqrt.append(prd_sqrt / num_of_iter)
        y_prd_default.append(prd_default / num_of_iter)
        y_prd_no_threads.append(prd_no_threads / num_of_iter)
        y_prd_invert.append(prd_invert / num_of_iter)
        y_prd_two_opt.append(prd_two_opt / num_of_iter)

        y_time_13.append(time_13 / num_of_iter)
        y_time_sqrt.append(time_sqrt / num_of_iter)
        y_time_default.append(time_default / num_of_iter)
        y_time_no_threads.append(time_no_threads / num_of_iter)
        y_time_invert.append(time_invert / num_of_iter)

    
      # Plots

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Comparison with 2-OPT / PRD')
    plt.plot(x, y_prd_two_opt, label='2-OPT')
    plt.plot(x, y_prd_default, label='tabu search')
    plt.legend()
    plt.savefig('results/plots/two_opt_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Tabu List Length / PRD')
    plt.plot(x, y_prd_13, label='13 elements')
    plt.plot(x, y_prd_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_prd_default, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Tabu List Length / TIME')
    plt.plot(x, y_time_13, label='13 elements')
    plt.plot(x, y_time_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_time_default, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_time.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Multithreading / PRD')
    plt.plot(x, y_prd_default, label='multiple threads')
    plt.plot(x, y_prd_no_threads, label='single thread')
    plt.legend()
    plt.savefig('results/plots/multithreading_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Multithreading / TIME')
    plt.plot(x, y_time_default, label='multiple threads')
    plt.plot(x, y_time_no_threads, label='single thread')
    plt.legend()
    plt.savefig('results/plots/multithreading_' + name.lower() + '_time.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Neighbourhood / PRD')
    plt.plot(x, y_prd_default, label='swap')
    plt.plot(x, y_prd_invert, label='invert')
    plt.legend()
    plt.savefig('results/plots/neighbours_' + name.lower() + '_prd.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Neighbourhood / TIME')
    plt.plot(x, y_time_default, label='swap')
    plt.plot(x, y_time_invert, label='invert')
    plt.legend()
    plt.savefig('results/plots/neighbours_' + name.lower() + '_time.png')

    # Latex tables

    cols = "|" + (3) * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_prd_default[k]) + " & " + str(y_prd_default[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"

    with open('results/tables/two_opt_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


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

    with open('results/tables/multithreading_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_time_default[k]) + " & " + str(y_time_no_threads[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"
    
    with open('results/tables/multithreading_' +  name.lower() + '_time.txt', 'w') as f:
        f.write(text)

    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_prd_default[k]) + " & " + str(y_prd_invert[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"

    with open('results/tables/neighbour_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


    cols = "|" + 3 * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_time_default[k]) + " & " + str(y_time_invert[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"
    
    with open('results/tables/neighbour_' +  name.lower() + '_time.txt', 'w') as f:
        f.write(text)


test_tsplib(files_euclid, pre_euclid, post_euclid, name_euclid)
test_tsplib(files_asym, pre_asym, post_asym, name_asym)
test_tabu(create_euclid, 'Euclid')
test_tabu(create_tsp, 'Symmetric')
test_tabu(create_atsp, 'Asymmetric')

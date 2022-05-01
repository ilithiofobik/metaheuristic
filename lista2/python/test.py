from cgi import test
from tsp_tabu import *
from datetime import datetime
from math import sqrt
import matplotlib.pyplot as plt


num_of_iter = 3
compared_options = 3


def test_tabu_length(gen, name):
    x = []

    y_prd_13 = []
    y_prd_sqrt = []
    y_prd_n = []

    y_time_13 = []
    y_time_sqrt = []
    y_time_n = []

    for n in range(50, 550, 50):
        time_13 = 0
        time_sqrt = 0
        time_n = 0

        prd_13 = 0
        prd_sqrt = 0
        prd_n = 0

        print(n)

        for _ in range(num_of_iter):
            m = gen(n)

            start_time = datetime.now()
            (val_13, _) = tabu_search(m, 13, True)
            time_13 = (datetime.now() - start_time).total_seconds() * 1000

            start_time = datetime.now()
            (val_sqrt, _) = tabu_search(m, int(sqrt(n)), True)
            time_sqrt = (datetime.now() - start_time).total_seconds() * 1000

            start_time = datetime.now()
            (val_n, _) = tabu_search(m, n, True)
            time_n = (datetime.now() - start_time).total_seconds() * 1000

            min_val = min(val_13, val_sqrt, val_n)
            prd_13 += ((val_13 - min_val) / min_val) * 100.0
            prd_sqrt += ((val_sqrt - min_val) / min_val) * 100.0
            prd_n += ((val_n - min_val) / min_val) * 100.0

        x.append(n)

        y_prd_13.append(prd_13 / num_of_iter)
        y_prd_sqrt.append(prd_sqrt / num_of_iter)
        y_prd_n.append(prd_n / num_of_iter)

        y_time_13.append(time_13 / num_of_iter)
        y_time_sqrt.append(time_sqrt / num_of_iter)
        y_time_n.append(time_n / num_of_iter)

    
    # Plots

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('PRD [%]')
    plt.title(name + ' / Tabu List Length / PRD')
    plt.plot(x, y_prd_13, label='13 elements')
    plt.plot(x, y_prd_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_prd_n, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_time.png')

    plt.clf()
    plt.xlabel('n')
    plt.ylabel('Time [milliseconds]')
    plt.title(name + ' / Tabu List Length / TIME')
    plt.plot(x, y_time_13, label='13 elements')
    plt.plot(x, y_time_sqrt, label='sqrt(n) elements')
    plt.plot(x, y_time_n, label='n elements')
    plt.legend()
    plt.savefig('results/plots/tabu_' + name.lower() + '_prd.png')


    # Latex tables

    cols = "|" + (compared_options + 1) * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_prd_13[k]) + " & " + str(y_prd_sqrt[k]) + " & " + str(y_prd_n[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"

    with open('results/tables/tabu_' +  name.lower() + '_prd.txt', 'w') as f:
        f.write(text)


    cols = "|" + (compared_options + 1) * "c|"
    text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n"
    for k in range(len(x)):
        text += str(x[k]) + " & " + str(y_time_13[k]) + " & " + str(y_time_sqrt[k]) + " & " + str(y_time_n[k]) + "\\\\\n\\hline\n"
    text += "\\end{tabular}\n\\end{center}\n"
    
    with open('results/tables/tabu_' +  name.lower() + '_time.txt', 'w') as f:
        f.write(text)



test_tabu_length(create_atsp, 'Asymmetric')
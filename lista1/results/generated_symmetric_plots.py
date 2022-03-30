import csv
from cmath import log

import matplotlib.pyplot as plt

plots_avg_time = csv.reader(open('generated_tsp_test_avg_time.txt', 'r'), delimiter=';')
plots_max_time = csv.reader(open('generated_tsp_test_max_time.txt', 'r'), delimiter=';')
plots_prd = csv.reader(open('generated_tsp_test_prd.txt', 'r'), delimiter=';')

x = []
y_avg_time_rand  = []
y_avg_time_neigh  = []
y_avg_time_ext_neigh  = []
y_avg_time_opt  = []
y_max_time_rand  = []
y_max_time_neigh  = []
y_max_time_ext_neigh  = []
y_max_time_opt  = []
y_avg_prd_rand  = []
y_avg_prd_neigh  = []
y_avg_prd_ext_neigh  = []
y_avg_prd_opt  = []

for row in plots_avg_time:
    n = int(row[0])
    x.append(n)
    y_avg_time_rand.append(float(row[1]))
    y_avg_time_neigh.append(float(row[2]))
    y_avg_time_ext_neigh.append(float(row[3]))
    y_avg_time_opt.append(float(row[4]))

for row in plots_max_time:
    y_max_time_rand.append(float(row[1]))
    y_max_time_neigh.append(float(row[2]))
    y_max_time_ext_neigh.append(float(row[3]))
    y_max_time_opt.append(float(row[4]))

for row in plots_prd:
    y_avg_prd_rand.append(float(row[1]))
    y_avg_prd_neigh.append(float(row[2]))
    y_avg_prd_ext_neigh.append(float(row[3]))
    y_avg_prd_opt.append(float(row[4]))

plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Random Symmetric Plot - Avg Time')
plt.plot(x, y_avg_time_rand, label='1000-Random')
plt.plot(x, y_avg_time_neigh, label='Neighbours')
plt.plot(x, y_avg_time_ext_neigh, label='Extended Neighbours')
plt.plot(x, y_avg_time_opt, label='2-OPT')
plt.legend()
plt.savefig('generated_sym_avg_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Random Symmetric Plot - Max Time')
plt.plot(x, y_max_time_rand, label='1000-Random')
plt.plot(x, y_max_time_neigh, label='Neighbours')
plt.plot(x, y_max_time_ext_neigh, label='Extended Neighbours')
plt.plot(x, y_max_time_opt, label='2-OPT')
plt.legend()
plt.savefig('generated_sym_max_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD [%]')
plt.title('Random Symmetric Plot - Avg PRD')
plt.plot(x, y_avg_prd_rand, label='1000-Random')
plt.plot(x, y_avg_prd_neigh, label='Neighbours')
plt.plot(x, y_avg_prd_ext_neigh, label='Extended Neighbours')
plt.plot(x, y_avg_prd_opt, label='2-OPT')
plt.legend()
plt.savefig('generated_sym_avg_prd.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD [%]')
plt.title('Random Symmetric Plot - Avg PRD')
plt.plot(x, y_avg_prd_neigh, label='Neighbours')
plt.plot(x, y_avg_prd_ext_neigh, label='Extended Neighbours')
plt.plot(x, y_avg_prd_opt, label='2-OPT')
plt.legend()
plt.savefig('generated_sym_avg_prd_no_krandom.png')
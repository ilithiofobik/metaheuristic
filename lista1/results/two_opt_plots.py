import csv
from cmath import log

import matplotlib.pyplot as plt

plots_tsp = csv.reader(open('two_opt_tsp_test.txt', 'r'), delimiter=';')
plots_atsp = csv.reader(open('two_opt_atsp_test.txt', 'r'), delimiter=';')
plots_euclid = csv.reader(open('two_opt_euclid_test.txt', 'r'), delimiter=';')

x = []
y_avg_time_tsp_rand  = []
y_avg_time_tsp_ext_neigh  = []
y_max_time_tsp_rand  = []
y_max_time_tsp_ext_neigh  = []
y_avg_prd_tsp_rand  = []
y_avg_prd_tsp_ext_neigh  = []

y_avg_time_atsp_rand  = []
y_avg_time_atsp_ext_neigh  = []
y_max_time_atsp_rand  = []
y_max_time_atsp_ext_neigh  = []
y_avg_prd_atsp_rand  = []
y_avg_prd_atsp_ext_neigh  = []

y_avg_time_euclid_rand  = []
y_avg_time_euclid_ext_neigh  = []
y_max_time_euclid_rand  = []
y_max_time_euclid_ext_neigh  = []
y_avg_prd_euclid_rand  = []
y_avg_prd_euclid_ext_neigh  = []

for row in plots_tsp:
    n = int(row[0])
    x.append(n)
    y_avg_time_tsp_rand.append(float(row[1]))
    y_avg_time_tsp_ext_neigh.append(float(row[2]))
    y_max_time_tsp_rand.append(float(row[3]))
    y_max_time_tsp_ext_neigh.append(float(row[4]))
    y_avg_prd_tsp_rand.append(float(row[5]))
    y_avg_prd_tsp_ext_neigh.append(float(row[6]))

for row in plots_atsp:
    y_avg_time_atsp_rand.append(float(row[1]))
    y_avg_time_atsp_ext_neigh.append(float(row[2]))
    y_max_time_atsp_rand.append(float(row[3]))
    y_max_time_atsp_ext_neigh.append(float(row[4]))
    y_avg_prd_atsp_rand.append(float(row[5]))
    y_avg_prd_atsp_ext_neigh.append(float(row[6]))

for row in plots_euclid:
    y_avg_time_euclid_rand.append(float(row[1]))
    y_avg_time_euclid_ext_neigh.append(float(row[2]))
    y_max_time_euclid_rand.append(float(row[3]))
    y_max_time_euclid_ext_neigh.append(float(row[4]))
    y_avg_prd_euclid_rand.append(float(row[5]))
    y_avg_prd_euclid_ext_neigh.append(float(row[6]))


plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Symmetric 2-OPT Plot - Avg Time')
plt.plot(x, y_avg_time_tsp_rand, label='With 1000-Random')
plt.plot(x, y_avg_time_tsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_sym_avg_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Symmetric 2-OPT Plot - Max Time')
plt.plot(x, y_max_time_tsp_rand, label='With 1000-Random')
plt.plot(x, y_max_time_tsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_sym_max_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD [%]')
plt.title('Symmetric 2-OPT Plot - Avg PRD')
plt.plot(x, y_avg_prd_tsp_rand, label='With 1000-Random')
plt.plot(x, y_avg_prd_tsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_sym_avg_prd.png')


plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Asymmetric 2-OPT Plot - Avg Time')
plt.plot(x, y_avg_time_atsp_rand, label='With 1000-Random')
plt.plot(x, y_avg_time_atsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_asym_avg_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Asymmetric 2-OPT Plot - Max Time')
plt.plot(x, y_max_time_atsp_rand, label='With 1000-Random')
plt.plot(x, y_max_time_atsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_asym_max_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD [%]')
plt.title('Asymmetric 2-OPT Plot - Avg PRD')
plt.plot(x, y_avg_prd_atsp_rand, label='With 1000-Random')
plt.plot(x, y_avg_prd_atsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_asym_avg_prd.png')


plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Euclid 2-OPT Plot - Avg Time')
plt.plot(x, y_avg_time_atsp_rand, label='With 1000-Random')
plt.plot(x, y_avg_time_atsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_euclid_avg_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('Time [nanoseconds]')
plt.title('Euclid 2-OPT Plot - Max Time')
plt.plot(x, y_max_time_atsp_rand, label='With 1000-Random')
plt.plot(x, y_max_time_atsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_euclid_max_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD [%]')
plt.title('Euclid 2-OPT Plot - Avg PRD')
plt.plot(x, y_avg_prd_atsp_rand, label='With 1000-Random')
plt.plot(x, y_avg_prd_atsp_ext_neigh, label='With Extended Neighbours')
plt.legend()
plt.savefig('two_opt_euclid_avg_prd.png')
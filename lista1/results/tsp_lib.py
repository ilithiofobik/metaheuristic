import csv
import matplotlib.pyplot as plt

plots = csv.reader(open('tsp_lib_test.txt', 'r'), delimiter=';')

x = []
favg_prd_k_rand = []
favg_prd_ext_neigh = []
favg_prd_2_opt = []
avg_time_k_rand = []
avg_time_ext_neig = []
avg_time_2_opt = []
max_time_k_rand = []
max_time_ext_neig = []
max_time_2_opt = []


for row in plots:
    x.append(int(row[0]))
    favg_prd_k_rand.append(float(row[1]))
    favg_prd_ext_neigh.append(float(row[2]))
    favg_prd_2_opt.append(float(row[3]))
    avg_time_k_rand.append(int(row[4]))
    avg_time_ext_neig.append(int(row[5]))
    avg_time_2_opt.append(int(row[6]))
    max_time_k_rand.append(int(row[7]))
    max_time_ext_neig.append(int(row[8]))
    max_time_2_opt.append(int(row[9]))

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD(%)')
plt.title('Average PRD Plot')
plt.scatter(x, favg_prd_k_rand, label='1000-Random')
plt.scatter(x, favg_prd_ext_neigh, label='Extended-Neighbour')
plt.scatter(x, favg_prd_2_opt, label='2-OPT')
plt.legend()
plt.savefig('tsp_lib_prd.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('Time (ns)')
plt.title('Average Time Plot')
plt.scatter(x, avg_time_k_rand, label='1000-Random')
plt.scatter(x, avg_time_ext_neig, label='Extended-Neighbour')
plt.scatter(x, avg_time_2_opt, label='2-OPT')
plt.legend()
plt.savefig('tsp_lib_avg_time.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('Time (ns)')
plt.title('Maximum Time Plot')
plt.scatter(x, max_time_k_rand, label='1000-Random')
plt.scatter(x, max_time_ext_neig, label='Extended-Neighbour')
plt.scatter(x, max_time_2_opt, label='2-OPT')
plt.legend()
plt.savefig('tsp_lib_max_time.png')
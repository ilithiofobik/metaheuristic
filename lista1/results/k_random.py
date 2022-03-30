import csv
from cmath import log

import matplotlib.pyplot as plt

plots_sym = csv.reader(open('k_random_sym.txt', 'r'), delimiter=';')
plots_asym = csv.reader(open('k_random_asym.txt', 'r'), delimiter=';')
plots_euc = csv.reader(open('k_random_euc.txt', 'r'), delimiter=';')

x = []
y_sym10  = []
y_asym10  = []
y_euc10  = []
y_sym100  = []
y_asym100  = []
y_euc100  = []
y_sym1000  = []
y_asym1000  = []
y_euc1000  = []

for row in plots_sym:
    n = int(row[0])
    x.append(n)
    y_sym10.append(float(row[1]))
    y_sym100.append(float(row[2]))
    y_sym1000.append(float(row[3]))

for row in plots_asym:
    y_asym10.append(float(row[1]))
    y_asym100.append(float(row[2]))
    y_asym1000.append(float(row[3]))

for row in plots_euc:
    y_euc10.append(float(row[1]))
    y_euc100.append(float(row[2]))
    y_euc1000.append(float(row[3]))

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD')
plt.title('Symmetric PRD Plot')
plt.plot(x, y_sym10, label='10-Random')
plt.plot(x, y_sym100, label='100-Random')
plt.plot(x, y_sym1000, label='1000-Random')
plt.legend()
plt.savefig('sym_k_random.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD')
plt.title('Asymmetric PRD Plot')
plt.plot(x, y_asym10, label='10-Random')
plt.plot(x, y_asym100, label='100-Random')
plt.plot(x, y_asym1000, label='1000-Random')
plt.legend()
plt.savefig('asym_k_random.png')

plt.clf()
plt.xlabel('n')
plt.ylabel('PRD')
plt.title('Euclid PRD Plot')
plt.plot(x, y_euc10, label='10-Random')
plt.plot(x, y_euc100, label='100-Random')
plt.plot(x, y_euc1000, label='1000-Random')
plt.legend()
plt.savefig('euc_k_random.png')
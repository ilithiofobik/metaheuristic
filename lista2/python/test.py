from tsp_tabu import *
from datetime import datetime
from math import sqrt

num_of_iter = 7


def test_tabu_length():
    for n in range(50, 550, 50):
        time_13 = 0
        time_sqrt = 0
        time_n = 0

        val_13 = 0
        val_sqrt = 0
        val_n = 0

        for _ in range(num_of_iter):
            m = create_atsp(n)

            start_time = datetime.now()
            (val, _) = tabu_search(m, 13, True)
            time_13 = (datetime.now() - start_time).total_seconds() * 1000
            val_13 += val

            start_time = datetime.now()
            (val, _) = tabu_search(m, int(sqrt(n)), True)
            time_sqrt = (datetime.now() - start_time).total_seconds() * 1000
            val_sqrt += val

            start_time = datetime.now()
            (val, _) = tabu_search(m, n, True)
            time_n = (datetime.now() - start_time).total_seconds() * 1000
            val_n += val

        val_13 /= num_of_iter
        val_sqrt /= num_of_iter
        val_n /= num_of_iter

        time_13 /= num_of_iter
        time_sqrt /= num_of_iter
        time_n /= num_of_iter

        print(n)
        print("13:  ", str(val_13), " ", str(time_13))
        print("sqrt:", str(val_sqrt), " ", str(time_sqrt))
        print("n :  ", str(val_n), " ", str(time_n))

test_tabu_length()
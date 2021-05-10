from pyo3_cheatsheet import range_sum
import random
import time


def benchmark(func, args):
    print(func(*args))
    t1 = time.time()
    for i in range(100):
        func(*args)
    print(time.time() - t1)

benchmark(range_sum, (1, 100))

from pyo3_cheatsheet import range_sum, bert_tokenize, range_sum_recursive
import random
import time


def benchmark(func, args):
    print(func(*args))
    t1 = time.time()
    for i in range(100):
        func(*args)
    print(time.time() - t1)
    

def py_range_sum_recursive(a, b):
    if a == b:
        return 0
    else:
        return a + py_range_sum_recursive(a + 1, b) 

# benchmark(range_sum, (1, 100))
# print(bert_tokenize("i am happy to go to fudan university"))
benchmark(range_sum_recursive, (1, 900, ))
benchmark(py_range_sum_recursive, (1, 900, ))

benchmark(range_sum, (1, 900, ))

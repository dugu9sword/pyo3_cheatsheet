from pyo3_cheatsheet import range_sum, bert_tokenize, range_sum_recursive, multiply, merge_count
import random
import time

# from transformers import BertTokenizer, BertTokenizerFast

# tkzr = BertTokenizer.from_pretrained("bert-base-uncased")
# tkzr_fast = BertTokenizerFast.from_pretrained("bert-base-uncased")


def benchmark(func, args):
    # print(func(*args))
    t1 = time.time()
    for i in range(100):
        func(*args)
    print(time.time() - t1)
    

def py_range_sum_recursive(a, b):
    if a == b:
        return 0
    else:
        return a + py_range_sum_recursive(a + 1, b) 

def py_multiply(a, b):
    c = []
    for ea, eb in zip(a, b):
        c.append(ea * eb)
    return c

def tokenize(sent):
    return " ".join(tkzr.tokenize(sent))

def tokenize_fast(sent):
    return " ".join(tkzr_fast.tokenize(sent))

# benchmark(range_sum, (1, 100))
# print(bert_tokenize("i am happy to go to fudan university"))
# benchmark(range_sum_recursive, (1, 900, ))
# benchmark(py_range_sum_recursive, (1, 900, ))

# benchmark(range_sum, (1, 900, ))
# sent = """the german army is continuing to lose weapons and ammunition, often without a trace. since the beginning of 2014, the armed forces has registered a loss of 39 weapons, 39 weapon components, and almost 20,000 rounds of ammunition. this is revealed in a classified listing of the german ministry of defense on which the redaktionsnetzwerk deutschland (rnd) ("editor network germany") has reported."""
# benchmark(tokenize, (sent, ))
# benchmark(tokenize_fast, (sent, ))
# benchmark(bert_tokenize, (sent, ))


x = [random.random()] * 10000
y = [random.random()] * 10000
benchmark(multiply, (x, y))
benchmark(py_multiply, (x, y))

# print(distance([1, 2, 3], [3, 4, 5]))
# print(merge_count({"a": 1, "b": 3}, {"b": 2}))
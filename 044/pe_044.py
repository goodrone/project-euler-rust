import platform
if platform.python_implementation() != "PyPy":
    print("Consider running under PyPy")

import itertools

def gen_pentagonal_numbers():
    for n in itertools.count(1):
        v = n * (3 * n - 1) // 2
        yield v

assert list(itertools.islice(gen_pentagonal_numbers(), 10)) == [1, 5, 12, 22, 35, 51, 70, 92, 117, 145]

s = set(itertools.islice(gen_pentagonal_numbers(), 10000))
ss = set(itertools.islice(gen_pentagonal_numbers(), 1000000))

for a in s:
    for b in s:
        if a > b:
            continue
        if (a + b) in ss and (b - a) in s:
            print(b - a)
            break
    else:
        continue
    break

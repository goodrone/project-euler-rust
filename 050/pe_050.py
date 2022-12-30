import itertools
import math

def gen_primes():
    for n in itertools.count(2):
        for d in range(2, int(math.sqrt(n)) + 1):
            if n % d == 0:
                break
        else:
            yield n

primes = []
for i, x in enumerate(gen_primes()):
    if x > 1_000_000:
        break
    primes.append(x)
primes_set = set(primes)

def try_sum(start, target):
    s = 0
    for n, i in enumerate(itertools.count(start)):
        p = primes[i]
        s += p
        if s > target:
            break
        if s in primes_set:
            yield n + 1, s

m = 0
ss = 0
for n in range(100):
    for num, s in try_sum(n, 1_000_000):
        if num > m:
            m = num
            ss = s
print(ss)

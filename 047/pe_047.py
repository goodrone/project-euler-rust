import itertools
import math

def factorize(n):
    factors = []
    for d in range(2, n // 2 + 1):
        while n % d == 0:
            n //= d
            factors.append(d)
    return set(factors)

assert factorize(645) == set([3, 5, 43])
assert factorize(644) == set([2, 7, 23])
assert factorize(646) == set([2, 17, 19])
assert factorize(1024) == set([2])
assert factorize(43 * 43) == set([43])

ndz = 0
ndy = 0
ndx = 0
arr = []
for x in itertools.count(1):
    factors = factorize(x)
    nd = len(factors)
    arr.append(nd)
    if nd == 4 and ndx == 4 and ndy == 4 and ndz == 4:
        print(x - 3)
        break
    ndz = ndy
    ndy = ndx
    ndx = nd

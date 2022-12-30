import itertools

def get_4d_primes():
    result = []
    for n in range(1001, 9999):
        for x in range(2, n // 2 + 1):
            if n % x == 0:
                break
        else:
            result.append(n)
    return result

primes_4d = set(get_4d_primes())
assert len(primes_4d) == 1061

def gen_4d_numbers():
    for a in range(1, 10):
        for b in range(10):
            for c in range(10):
                for d in range(1, 10, 2):
                    yield a * 1000 + b * 100 + c * 10 + d, (str(a), str(b), str(c), str(d))

cp = set()
for n, a in gen_4d_numbers():
    if not n in primes_4d:
        continue
    pp = []
    for aa in itertools.permutations(a):
        x = int("".join(aa))
        if x in primes_4d:
            pp.append(x)
    if len(pp) >= 2:
        assert(n in pp)
        for p in pp:
            if p == n:
                continue
            d = abs(p - n)
            if 2 * p - n in pp:
                cp.add("".join(map(str, sorted((2 * p - n, p, n)))))
print((cp - set([148748178147])).pop())

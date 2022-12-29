import itertools

def gen_triangle():
    for n in itertools.count(1):
        v = n * (n + 1) // 2
        yield v

def gen_pentagonal():
    for n in itertools.count(1):
        v = n * (3 * n - 1) // 2
        yield v

def gen_hexagonal():
    for n in itertools.count(1):
        v = n * (2 * n - 1)
        yield v

t = gen_triangle()
p = gen_pentagonal()
h = gen_hexagonal()

def adv(i, n):
    for x in range(n):
        next(i)

adv(t, 284)
assert next(t) == 40755
adv(p, 164)
assert next(p) == 40755
adv(h, 142)
assert next(h) == 40755

nt = next(t)
np = next(p)
nh = next(h)
while True:
    if nt == np == nh:
        print(nt)
        break
    if nt < nh:
        nt = next(t)
    if np < nh:
        np = next(p)
    if nh < nt or nh < np:
        nh = next(h)

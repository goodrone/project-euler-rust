import collections

pp = collections.defaultdict(int)
for p in range(120, 1001):
    #print(p // 3 + 1)
    for a in range(1, p // 3 + 1):
        aa = a * a
        for b in range(a + 1, p):
            c = p - a - b
            d = aa + b * b - c * c
            if d == 0:
                pp[p] += 1
            elif d > 0:
                #print(a, b, c, d)
                break
    #break
m = 0
n = 0
for k, v in pp.items():
    if v > m:
        n = k
        m = v
print(n, m)

import bisect
import math

ai = [0]  # array of start indices
am = [1]  # array of magnitudes (number of digits)
#  ai, am will go like this:
#   0   1
#  10   2
# 190   3
for nd in range(1, 10):
    d = 9 * int(math.pow(10, nd - 1))
    if nd == 1:
        # special case to support "0"
        d += 1
    ai.append(ai[-1] + d * nd)
    am.append(nd + 1)

dd = 1
def get_d(lookup):
    i = bisect.bisect_left(ai, lookup) - 1
    local = lookup - ai[i]
    if am[i] == 1:
        n = local
    else:
        n = int(math.pow(10, am[i] - 1)) + local // am[i]
    d = int(str(n)[local % am[i]])
    return d

assert get_d(1) == 1
assert get_d(10) == 1
assert get_d(11) == 0
assert get_d(12) == 1

for x in range(7):
    lookup = int(math.pow(10, x))
    d = get_d(lookup)
    dd *= d
print(dd)

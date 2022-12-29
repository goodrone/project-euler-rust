import itertools
s = 0
for x in itertools.permutations(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]):
    n = int("".join(x))
    if (n % 1000) % 17 != 0:
        continue
    if ((n // 10) % 1000) % 13 != 0:
        continue
    if ((n // 100) % 1000) % 11 != 0:
        continue
    if ((n // 1000) % 1000) % 7 != 0:
        continue
    if ((n // 10000) % 1000) % 5 != 0:
        continue
    if ((n // 100000) % 1000) % 3 != 0:
        continue
    if ((n // 1000000) % 1000) % 2 != 0:
        continue
    s += n
print(s)

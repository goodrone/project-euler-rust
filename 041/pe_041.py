import itertools
import math

def is_prime(n):
    for d in range(2, int(math.sqrt(n)) + 1):
        if n % d == 0:
            return False
    return True

# manually selected 7 as the top number as 8 and 9 were not giving anything
for p in itertools.permutations(["7", "6", "5", "4", "3", "2", "1"]):
    n = int("".join(p))
    if is_prime(n):
        print(n)
        break

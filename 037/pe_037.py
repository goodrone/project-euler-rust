def download_primes():
    filename = "primes.txt"
    try:
        with open(filename, "r") as f:
            lines = f.readlines()
    except:
        url = "https://primes.utm.edu/lists/small/100000.txt"
        import urllib.request, os
        urllib.request.urlretrieve(url, ".tmp")
        os.rename(".tmp", filename)
        with open(filename, "r") as f:
            lines = f.readlines()
    lines = lines[5:]
    result = []
    for line in lines:
        result += map(int, line.split())
    return result

primes = download_primes()

import bisect

def check(n, left):
    s = str(n)
    if left:
        t = int(s[1:])
        is_tr = is_tr_left
    else:
        t = int(s[:-1])
        is_tr = is_tr_right
    i = bisect.bisect_left(primes, t)
    if i != len(primes) and primes[i] == t and (t < 10 or is_tr[i]):
        index = bisect.bisect_left(primes, n)
        is_tr[index] = True
        return True
    return False

result = []
is_tr_left = [False] * len(primes)
is_tr_right = [False] * len(primes)
for i, p in enumerate(primes):
    if p < 10:
        continue
    left = check(p, True)
    right = check(p, False)
    if left and right:
        #is_truncatable[i] = True
        result.append(p)
    continue

print(sum(result))

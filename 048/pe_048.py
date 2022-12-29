def pow_lim(a, b, lim):
    result = a
    for x in range(b - 1):
        result = (result * a) % lim
    return result

assert pow_lim(3, 3, 10000) == 3 * 3 * 3

lim = 10_000_000_000
result = 0
for x in range(1, 1001):
    result += pow_lim(x, x, lim) 
print(result % lim)

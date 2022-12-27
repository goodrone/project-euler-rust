def format_number(n):
    if n < 1 or n > 1000:
        raise RuntimeError(f"Number out of range: {n}")
    if n == 1000:
        return "one thousand"
    d3 = n // 100
    d2 = n // 10 % 10
    d1 = n % 10
    tokens = []
    if d3:
        tokens += [LOOKUP[d3], "hundred"]
        if d2 or d1:
            tokens.append("and")
    if d2 == 1:
        tokens.append(LOOKUP[d2 * 10 + d1])
    if d2 >= 2:
        tokens.append(LOOKUP[d2 * 10])
    if d1 and d2 != 1:
        tokens.append(LOOKUP[d1])
    return " ".join(tokens)

count = 0
for x in range(1, 1001):
    p = sum(map(len, format_number(x).split()))
    count += p
print(count)

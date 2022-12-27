import itertools

def permute_9():
    for p in itertools.permutations(["1", "2", "3", "4", "5", "6", "7", "8"]):
        n = "9" + "".join(p)
        yield n

result = []

for s in permute_9():
    for nd in range(1, 5):
        prefix = int(s[:nd])
        c = str(prefix)
        for x in range(2, 6):
            c += str(prefix * x)
            if len(c) == 9 and c == s:
                result.append(int(s))
            elif len(c) > 9:
                break

print(max(result))

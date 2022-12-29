with open("p042_words.txt", "r") as f:
    data = f.read()

import json
words = json.loads("[" + data + "]")
assert len(words) == 1786

def calc_word_number(w):
    result = 0
    base = ord('A')
    for c in w:
        value = ord(c) - base + 1
        result += value
    return result

assert calc_word_number("SKY") == 55

word_numbers = list(map(calc_word_number, words))
max_num = max(word_numbers)

import itertools

def gen_triangular_numbers():
    for n in itertools.count(1):
        t = n * (n + 1) // 2
        yield t

assert list(itertools.islice(gen_triangular_numbers(), 10)) == [1, 3, 6, 10, 15, 21, 28, 36, 45, 55]

triangular_numbers = set()
for t in gen_triangular_numbers():
    if t > max_num:
        break
    triangular_numbers.add(t)

count = 0
for n in word_numbers:
    if n in triangular_numbers:
        count += 1
print(count)

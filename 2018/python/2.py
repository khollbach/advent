from collections import Counter

# Part 1

# Read lines and strip whitespace
ids = list(map(str.strip, open('../inputs/2').readlines()))

# Does s have any char with exactly freqency freq?
def has_char_with_freq(freq, s):
    return any(f == freq for f in Counter(s).values())

# (num words with a 2-frequency-chararacter)
# * (num words with a 3-frequency-chararacter)
print(sum(map(lambda x: 1, filter(lambda s: has_char_with_freq(2, s), ids)))
    * sum(map(lambda x: 1, filter(lambda s: has_char_with_freq(3, s), ids))))

# Part 2

# Two strings are similar if they differ in exactly one column.
def similar(x, y):
    if len(x) != len(y):
        return False
    differences = 0
    for i, c in enumerate(x):
        if c != y[i]:
            if differences > 0:
                return False
            differences += 1
    return differences == 1

for x in ids:
    for y in ids:
        if similar(x, y):
            print(x)
            print(y)
            import sys
            sys.exit()

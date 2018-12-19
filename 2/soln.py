#!/usr/bin/python3

from collections import Counter

# Part 1

# Read lines and strip whitespace
ids = list(map(str.strip, open('input').readlines()))

# Does s have any char with exactly freqency freq?
def has_char_with_freq(freq, s):
    return any(f == freq for f in Counter(s).values())

# (num words with a 2-frequency-chararacter)
# * (num words with a 3-frequency-chararacter)
print(sum(map(lambda x: 1, filter(lambda s: has_char_with_freq(2, s), ids)))
    * sum(map(lambda x: 1, filter(lambda s: has_char_with_freq(3, s), ids))))

# Part 2

print('TODO')
assert False

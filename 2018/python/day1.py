# Part 1

f = open('../inputs/1')

freq = 0
for s in f.readlines():
    freq += int(s)

print(freq)

# Part 2

changes = []
for s in open('../inputs/1').readlines():
    changes.append(int(s))

freq = 0
freqs_seen = set([0])

i = 0
while True:
    freq += changes[i % len(changes)]

    if freq in freqs_seen:
        print(freq)
        break

    freqs_seen.add(freq)

    i += 1

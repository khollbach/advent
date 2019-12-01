def part1():
    # e.g., s == "TCZOXuUEebQgFfGTscCSt"
    s = open('../inputs/5').read().strip()

    print(reacted_length(s))

def part2():
    s = open('../inputs/5').read().strip()

    best_l = len(s)

    c = 'a'
    while c <= 'z':
        s2list = []
        for char in s:
            if char not in (c, c.upper()):
                s2list.append(char)
        s2 = ''.join(s2list)

        l = reacted_length(s2)

        if l < best_l:
            best_l = l

        c = chr(ord(c) + 1)

    print(best_l)

def reacted_length(s):
    # React s
    i = 0
    while i + 1 < len(s):
        if reacts(s[i], s[i+1]):
            s = s[:i] + s[i+2:]

            # Hop back, so you don't miss a revealed reaction
            if i > 0:
                i -= 1
        else:
            i += 1

    return len(s)

def reacts(c1, c2):
    return c1 != c2 and c1.lower() == c2.lower()

part1()
part2()

#!/usr/bin/python3

def main():
    # e.g., s == "TCZOXuUEebQgFfGTscCSt"
    s = open('input').read().strip()

    print(len(s))

    i = 0
    while i + 1 < len(s):
        if reacts(s[i], s[i+1]):
            s = s[:i] + s[i+2:]

            # Hop back, so you don't miss a revealed reaction
            if i > 0:
                i -= 1
        else:
            i += 1

    print(len(s))

def reacts(c1, c2):
    return c1 != c2 and c1.lower() == c2.lower()

main()

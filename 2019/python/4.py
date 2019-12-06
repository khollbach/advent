import doctest

def is_valid_part_1(password: str) -> bool:
    """
    Return True iff the password is valid (except for range checks).

    >>> is_valid_part_1("111111")
    True
    >>> is_valid_part_1("223450")
    False
    >>> is_valid_part_1("123789")
    False
    """
    has_repeating = False
    for i in range(1, len(password)):
        curr, prev = password[i], password[i - 1]
        if curr < prev:
            return False
        elif curr == prev:
            has_repeating = True
    return has_repeating

def is_valid_part_2(password: str) -> bool:
    """
    Return True iff the password is valid (except for range checks).

    >>> is_valid_part_2("111111")
    False
    >>> is_valid_part_2("223450")
    False
    >>> is_valid_part_2("123789")
    False
    >>> is_valid_part_2("112233")
    True
    >>> is_valid_part_2("123444")
    False
    >>> is_valid_part_2("111122")
    True
    """
    has_repeating = False
    for i in range(1, len(password)):
        prev, curr = password[i - 1], password[i]
        if prev > curr:  # Inversion --> bad.
            return False
        elif curr == prev and \
            (i - 2 < 0 or curr != password[i - 2]) and \
            (i + 1 >= len(password) or curr != password[i + 1]):

            has_repeating = True  # "Island" of length exactly two --> good.
    return has_repeating

if __name__ == "__main__":
    doctest.testmod()

    # Inputs:
    low = 136760
    high = 595730

    print(sum(is_valid_part_1(str(i)) for i in range(low, high + 1)))
    print(sum(is_valid_part_2(str(i)) for i in range(low, high + 1)))

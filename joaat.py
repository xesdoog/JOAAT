def joaat(key: str) -> int:
    key = key.lower()
    hash = 0

    for i in range(0, len(key)):
        hash += ord(key[i])
        hash += (hash << 10)
        hash = hash & 0xFFFFFFFF
        hash = hash ^ (hash >> 6)

    hash += (hash << 3)
    hash = hash & 0xFFFFFFFF
    hash = hash ^ (hash >> 11)
    hash += (hash << 15)
    hash = hash & 0xFFFFFFFF

    return hash

assert(joaat("adder") == 0xB779A091)
assert(joaat("ADDER") == 0xB779A091)

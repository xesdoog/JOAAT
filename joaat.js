function joaat(key)
{
    key = key.toLowerCase();
    let hash = 0;

    for (let i = 0; i < key.length; i++)
    {
        hash += key.charCodeAt(i);
        hash += (hash << 10);
        hash ^= (hash >>> 6);
        hash >>>= 0;
    }

    hash += (hash << 3);
    hash ^= (hash >>> 11);
    hash += (hash << 15);

    return hash >>> 0;
}


console.assert(joaat("adder") === 0xB779A091, "Test failed");
console.assert(joaat("ADDER") === 0xB779A091, "Case insensitivity test failed");

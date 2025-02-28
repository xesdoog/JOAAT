using System.Diagnostics;

public static class JoaatHasher
{
    public static uint joaat(string key)
    {
        key = key.ToLower();
        uint hash = 0;

        foreach (char c in key)
        {
            hash += (uint)c;
            hash += hash << 10;
            hash ^= hash >> 6;
        }

        hash += hash << 3;
        hash ^= hash >> 11;
        hash += hash << 15;

        return hash;
    }
}

Debug.Assert(JoaatHasher.joaat("adder") == 0xB779A091);
Debug.Assert(JoaatHasher.joaat("ADDER") == 0xB779A091);

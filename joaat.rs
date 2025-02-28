pub fn joaat(key: &str) -> u32
{
    let key = key.to_lowercase();
    let mut hash: u32 = 0;

    for byte in key.as_bytes()
    {
        hash = hash.wrapping_add(*byte as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }

    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);

    hash
}

#[test]
fn test_joaat() {
    assert_eq!(joaat("adder"), 0xB779A091);
    assert_eq!(joaat("ADDER"), 0xB779A091);
}
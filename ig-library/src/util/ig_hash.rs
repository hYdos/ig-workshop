/// uses the Fnv1a 32bit algorithm to hash an input string
pub fn hash(input: &str) -> u32 {
    let mut num1 = 2166136261u32;
    for num2 in input.as_bytes() {
        num1 = 16777619_u32.wrapping_mul(num1 ^ *num2 as u32);
    }

    num1
}

/// converts input string into lower case then calls (hash)[hash]
pub fn hash_lower(str: &str) -> u32 {
    let str = str.to_lowercase();
    hash(&str)
}

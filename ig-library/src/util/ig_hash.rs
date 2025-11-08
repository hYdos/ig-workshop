use std::collections::HashMap;
use std::sync::OnceLock;
use phf::phf_map;

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

static HASHES: OnceLock<HashMap<u32, &'static str>> = OnceLock::new();

/// Sometimes, we are debugging code, and we want common names to be readable so we can fix issues faster. I'm putting common words/phrases in here so we can check against them easier.
pub fn debug_decode_hash(h: u32) -> String {
    let map = HASHES.get_or_init(|| {
        HashMap::from([
            (hash("System"), "System"),
            (hash("level.bld"), "level.bld"),
            (hash("Game Constants"), "Game Constants"),
        ])
    });

    if let Some(name) = map.get(&h) {
        name.to_string()
    } else {
        format!("{:x}", h)
    }
}

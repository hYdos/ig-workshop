use std::hash::{Hash, Hasher};

struct Fnv32Hasher(u32);

impl Hasher for Fnv32Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let Fnv32Hasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u32);
            hash = hash.wrapping_mul(16777619);
        }

        *self = Fnv32Hasher(hash);
    }
}

/// uses the Fnv1a 32bit algorithm to hash an input string
pub fn hash(input: &str) -> u32 {
    let mut num1 = 2166136261u32;
    for num2 in input.as_bytes() {
        num1 = 16777619_u32.wrapping_mul(num1 ^ *num2 as u32);
    }
    
    num1
    // 
    // let mut hasher = Fnv32Hasher { 0: 2166136261 };
    // input.hash(&mut hasher);
    // hasher.finish() as u32
}

/// converts input string into lower case then calls (hash)[hash]
pub fn hash_lower(str: &str) -> u32 {
    let str = str.to_lowercase();
    hash(&str)
}


use crate::util::ig_hash::hash_lower;

pub struct igName {
    pub string: Option<String>,
    pub hash: u32,
}

impl igName {
    pub fn new(string: String) -> Self {
        igName {
            hash: hash_lower(&string),
            string: Some(string),
        }
    }
}

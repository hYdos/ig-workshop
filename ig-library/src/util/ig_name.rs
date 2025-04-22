use crate::util::ig_hash::hash_lower;

#[derive(Debug, Clone)]
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
    
    pub fn from_hash(hash: u32) -> Self {
        igName {
            hash,
            string: None,
        }
    }
}

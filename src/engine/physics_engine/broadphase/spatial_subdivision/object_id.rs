
pub struct ObjectId {
    pub control_bits: u8,
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let byte_str = format!("{:08b}", self.control_bits);
        write!(f, "{}_{}", &byte_str[0..4], &byte_str[4..])
    }
}

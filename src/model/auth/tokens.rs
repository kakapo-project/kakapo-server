
use uuid::Uuid;
use std::fmt;
use base64;
use rand::rngs::OsRng;
use rand::RngCore;
use rand::Error;

#[derive(Clone, Debug)]
pub struct Token {
    bytes: Vec<u8>,
}

impl Token {
    pub fn new() -> Result<Self, Error> {
        let mut rng = OsRng::new()?;
        let mut bytes = vec![];
        for _ in 0..4 {
            let next_int = rng.next_u32();

            bytes.push((next_int & 0xFF) as u8);
            bytes.push((next_int >> 8 & 0xFF) as u8);
            bytes.push((next_int >> 16 & 0xFF) as u8);
            bytes.push((next_int >> 24 & 0xFF) as u8);
        }

        Ok(Self { bytes })
    }

    pub fn as_string(&self) -> String {
        base64::encode(&self.bytes)
    }
}

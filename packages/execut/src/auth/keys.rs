use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        let encoding = EncodingKey::from_secret(secret);
        let decoding = DecodingKey::from_secret(secret);

        Self { encoding, decoding }
    }
}

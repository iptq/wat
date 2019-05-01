use rand::{thread_rng, Rng};
use serde::{Deserialize, Deserializer, Serializer};

/// Generate a random secret key.
pub fn generate_secret_key() -> Vec<u8> {
    let mut rng = thread_rng();
    let bytes = rng.gen::<[u8; 32]>();
    bytes.to_vec()
}

pub fn as_hex<S>(key: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(&key[..]))
}

pub fn from_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .map(|string| hex::decode(&string).expect("hex failed to decode"))
}

use hex::FromHexError;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct Config {
    #[serde(serialize_with = "as_hex", deserialize_with = "from_hex")]
    #[derivative(Default(value = "generate_secret_key()"))]
    pub secret_key: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum DatabaseUrl {
    Mysql(String),
    Postgres(String),
    Sqlite(String),
}

/// Generate a random secret key.
fn generate_secret_key() -> Vec<u8> {
    let mut rng = thread_rng();
    let bytes = rng.gen::<[u8; 32]>();
    bytes.to_vec()
}

fn as_hex<S>(key: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hex::encode(&key[..]))
}

fn from_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .map(|string| hex::decode(&string).expect("hex failed to decode"))
}

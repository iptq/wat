use std::io;

use crate::utils::{as_hex, from_hex, generate_secret_key};

#[derive(Debug, Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct Config {
    #[serde(serialize_with = "as_hex", deserialize_with = "from_hex")]
    #[derivative(Default(value = "generate_secret_key()"))]
    pub secret_key: Vec<u8>,

    pub database_url: String,
}

impl Config {
    pub fn read() -> Result<Self, io::Error> {
        Ok(Self::default())
    }
}

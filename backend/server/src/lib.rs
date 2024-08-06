use axum::extract::FromRef;
use socialverse_query::{AsyncConnection, AsyncConnectionPool, QueryError};

pub mod logging;
pub mod router;
pub mod error;
pub mod extractor;
pub mod handler;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: AsyncConnectionPool,
    pub signing_keys: socialverse_crypto::sign::Keys,
    pub rng: rand::rngs::StdRng,
}

impl AppState {
    pub async fn connect(&self) -> Result<AsyncConnection, QueryError> {
        self.db_pool.get().await
    }
    
}

pub mod cli {
    use color_eyre::{eyre::Context, Help};
    use rand::{CryptoRng, RngCore};
    use socialverse_crypto::sign::{encode_private_key, EncodedPrivateKey, Keys};

    pub fn gen_keys<R>(rng: &mut R) -> color_eyre::Result<(EncodedPrivateKey, Keys)>
    where
        R: CryptoRng + RngCore,
    {
        let (private_key, keys) = Keys::generate(rng)?;
        let private_key = encode_private_key(private_key)?;
        println!("private key from gen_keys: {:?}", private_key);
        Ok((private_key, keys))
    }

    pub fn load_keys() -> color_eyre::Result<Keys> {
        let private_key = std::env::var("API_PRIVATE_KEY")
            .wrap_err("failed to locate private API key")
            .suggestion("set API_PRIVATE_KEY environment variable")?;

        Ok(Keys::from_encoded(private_key)?)
    }
}

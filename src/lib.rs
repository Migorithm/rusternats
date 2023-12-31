use std::sync::OnceLock;

use async_nats::jetstream::Context;
use serde::{Deserialize, Serialize};

// Test Data that's to be serialized and deserialized
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomData {
    pub name: String,
    pub age: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccountCreated {
    pub(crate) id: String,
    pub(crate) user_name: String,
    pub(crate) jwt_token: String,
}

// Minimal client implementation
pub async fn nats_stream() -> &'static Context {
    static NATS_CLIENT: OnceLock<Context> = OnceLock::new();
    if NATS_CLIENT.get().is_none() {
        let client = async_nats::connect("localhost:4222").await.unwrap();

        NATS_CLIENT.get_or_init(|| async_nats::jetstream::new(client));
    }
    NATS_CLIENT.get().unwrap()
}

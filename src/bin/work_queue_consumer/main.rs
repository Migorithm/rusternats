use async_nats::jetstream::consumer::pull::Config;
use async_nats::jetstream::consumer::Consumer;

use futures::TryStreamExt;

use rand::seq::SliceRandom;
use serde_json::Value;

use std::sync::OnceLock;
use std::time::Duration;

use queue_test::nats_stream;

pub async fn nats_consumer() -> &'static Consumer<Config> {
    static NATS_CLIENT: OnceLock<Consumer<Config>> = OnceLock::new();
    let context = nats_stream().await;

    /*
     * `duplicate_window: Duration::from_secs(30)` is the period of time where the system keeps track of duplicate messages
     * so it can guarantee `exactly once` semantic.
     */
    let stream = context
        .create_stream(async_nats::jetstream::stream::Config {
            name: "events".to_string(),
            retention: async_nats::jetstream::stream::RetentionPolicy::WorkQueue,
            duplicate_window: Duration::from_secs(60),
            subjects: vec!["events.>".to_string()],
            ..Default::default()
        })
        .await
        .unwrap();

    let consumer_name = ["consumer-2", "consumer-1"]
        .choose(&mut rand::thread_rng())
        .unwrap();

    let mut filter_subject = "".to_string();
    if *consumer_name == "consumer-1" {
        filter_subject += "events.email.1"
    } else {
        filter_subject += "events.sms.2"
    }

    let consumer = stream
        .get_or_create_consumer(
            consumer_name,
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some(consumer_name.to_string()),
                filter_subject,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    NATS_CLIENT.get_or_init(|| consumer)
}

#[tokio::main]
async fn main() {
    let consumer = nats_consumer().await;

    let mut messages = consumer.messages().await.unwrap();

    while let Ok(Some(message)) = messages.try_next().await {
        // Deserialize the message payload into a Payload value.
        // let payload: Payload = serde_json::from_slice(message.payload.as_ref())?;

        println!("{}", message.message.subject);
        if let Ok(payload) = serde_json::from_slice::<Value>(&message.payload) {
            use rand::seq::SliceRandom;

            if ![true, false].choose(&mut rand::thread_rng()).unwrap() {
                message.double_ack().await.unwrap();
                println!("received payload: {:?}", payload,);
            }
        }
        // println!("{:?}", message)
    }
}

use async_nats::jetstream::consumer::pull::Config;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::consumer::DeliverPolicy;
use futures::TryStreamExt;

use std::sync::OnceLock;
use std::time::Duration;

use queue_test::nats_stream;
use queue_test::CustomData;

pub async fn nats_consumer() -> &'static Consumer<Config> {
    static NATS_CLIENT: OnceLock<Consumer<Config>> = OnceLock::new();
    let context = nats_stream().await;

    /*
     * `duplicate_window: Duration::from_secs(30)` is the period of time where the system keeps track of duplicate messages
     * so it can guarantee `exactly once` semantic.
     */
    let stream = context
        .get_or_create_stream(async_nats::jetstream::stream::Config {
            name: "events".to_string(),
            max_messages: 10_000,
            duplicate_window: Duration::from_secs(30),
            ..Default::default()
        })
        .await
        .unwrap();

    /*
     * `deliver_policy: DeliverPolicy::New` doesn't mean that the consumer will consume messages that are sent to server
     * only after it's `connected` to server. Rather, it means only after it's `created`.
     * So, it still keeps track of messages
     */
    let consumer = stream
        .get_or_create_consumer(
            "consumer",
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some("consumer".to_string()),
                deliver_policy: DeliverPolicy::New,
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
        if let Ok(payload) = serde_json::from_slice::<CustomData>(&message.payload) {
            println!(
                "received payload: name={:?} age={:?}",
                payload.name, payload.age
            );

            message.double_ack().await.unwrap();
        }
    }
}

// TODO Consume event
// TODO See if it's okay for consumer to be disconnected (done)
// TODO See if consumers are grouped together with the same `durable_name` (done)
// TODO See if when consumers are disconnected and turn back on track, it consumes the message from the point it left off(done.)

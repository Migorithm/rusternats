use async_nats::{header::NATS_MESSAGE_ID, HeaderMap};
use queue_test::{nats_stream, CustomData};
use rand::seq::SliceRandom;
use serde_json::json;

#[tokio::main]
async fn main() {
    let context = nats_stream().await;

    for i in 0..1000 {
        let payload = CustomData {
            name: "Meg9o04".to_string(),
            age: i,
        };

        let bytes = serde_json::to_vec(&json!(payload)).unwrap();

        // * Crucial if you want to avoid duplicated messages to be sent for the given `duplicate window`
        let mut map: HeaderMap = HeaderMap::new();

        map.insert(NATS_MESSAGE_ID, (i.to_string() + "so").as_str());

        let selected_subject = ["events.email.1", "events.sms.2", "events.somethingelse"]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        println!("selected subject: {}", selected_subject);
        let a = context
            .publish_with_headers(selected_subject, map, bytes.into())
            .await
            .unwrap();

        let a = a.await.map_err(|err| println!("{}", err));
    }

    println!("Send a message!")
}

// TODO Define event (done)
// TODO Send Json Serialized data(done)
// TODO See if it's okay for producer to be disconnected(done)
// TODO See if the message with the same id is not sent(done)

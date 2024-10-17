use futures::StreamExt;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, DefaultConsumerContext};
use rdkafka::message::Headers;
use rdkafka::Message;
use serde::Deserialize;

async fn consume_and_print(topics: &[&str]) {
    let server = std::env::var("KAFKA_SERVER").unwrap_or_else(|_| "localhost:9092".into());
    let username = std::env::var("KAFKA_USERNAME").unwrap_or_else(|_| "kafka".into());
    let password = std::env::var("KAFKA_PASSWORD").unwrap_or_else(|_| "kafka".into());
    let group_id = std::env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "1".into());

    let consumer: StreamConsumer<DefaultConsumerContext> = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", server)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", username)
        .set("sasl.password", password)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(DefaultConsumerContext)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    println!("Start subscribe message:");

    // consumer.stream() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!("Received Event:");

                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        println!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }
                match serde_json::from_str::<Currency>(payload) {
                    Ok(data) => {
                        println!("===================================");
                        if data.btc > 50000.0 {
                            println!("!!!Start to sell BTC at {}", data.btc);
                        } else {
                            println!("!!!Start to buy BTC at {}", data.btc);
                        }
                        println!("===================================");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
#[tokio::main]
async fn main() {
    let topics = std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "kong-upstream".into());
    consume_and_print(&[&topics]).await
}

#[derive(Debug, Deserialize)]
struct Currency {
    btc: f64,
}

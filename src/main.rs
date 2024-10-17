use serde_json::{json, Value};
use cloudevents::{EventBuilder, EventBuilderV10};
use cloudevents::binding::rdkafka::{FutureRecordExt, MessageRecord};
use rdkafka::config::{ClientConfig};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use std::net::SocketAddr;
use uuid::Uuid;

use axum::{
    async_trait,
    extract::FromRequest,
    http::{header::CONTENT_TYPE, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{post,get},
    Form, Json, RequestExt, Router,
};
use chrono::Utc;

struct JsonOrForm<T>(T);
#[async_trait]
impl<S, B, T> FromRequest<S, B> for JsonOrForm<T>
where
    B: Send + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<(), B>,
    Form<T>: FromRequest<(), B>,
    T: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}

async fn transfer_to_kafka(JsonOrForm(payload): JsonOrForm<Value>) -> Json<Value> {
    let server   = std::env::var("KAFKA_SERVER").unwrap_or_else(|_| "localhost:9092".into());
    let username = std::env::var("KAFKA_USERNAME").unwrap_or_else(|_| "kafka".into());
    let password = std::env::var("KAFKA_PASSWORD").unwrap_or_else(|_| "kafka".into());
    let topic    = std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "kong-upstream".into());
    let producer: &FutureProducer = &ClientConfig::new()
                .set("bootstrap.servers", server)
                .set("security.protocol", "SASL_SSL")
                .set("sasl.mechanisms", "PLAIN")
                .set("sasl.username", username)
                .set("sasl.password",password)
                .set("message.timeout.ms", "7000")
                .create()
                .expect("Producer creation error");

        let id = Uuid::new_v4();
        // The send operation on the topic returns a future, which will be
        // completed once the result or failure from Kafka is received.
        let event = EventBuilderV10::new()
            .id(id.to_string())
            .ty("example.test")
            .source("http://localhost/")
            .data("application/json", json!(payload))
            .time(Utc::now())
            .build()
            .unwrap();

        println!("Sending event: {:#?}", event);
        let message_record =
            MessageRecord::from_event(event).expect("error while serializing the event");

        let _delivery_status = producer
            .send(
                FutureRecord::to(&topic)
                    .message_record(&message_record)
                    .key(&format!("Key {}", id)),
                Duration::from_secs(0),
            )
            .await;
       //dbg!(_delivery_status);
       Json(json!({"message": "message sent"}))

}
#[tokio::main]
async fn main() {
    
     // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/send", post(transfer_to_kafka))   
    ;
    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// basic handler that responds with a static string
async fn root() -> &'static str {
    "Pong!"
}
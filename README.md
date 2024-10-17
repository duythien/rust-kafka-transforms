# Kafka Transforms

This service transforms requests into Kafka messages in an Apache Kafka topic. For more information, see Kafka topics.

# Rust lib

We use that lib to handel kafka message https://github.com/fede1024/rust-rdkafka

# Kafka Protocol Binding for CloudEvents - Version 1.0.2

This example shows the binary mode mapping of an event into the Kafka message. All other CloudEvents attributes are mapped to Kafka Header fields with prefix ce_.

Mind that ce_ here does refer to the event data content carried in the payload.

```
------------------ Message -------------------

Topic Name: mytopic

------------------- key ----------------------

Key: mykey

------------------ headers -------------------

content-type: application/cloudevents+json; charset=UTF-8

------------------- value --------------------

{
    "specversion" : "1.0",
    "type" : "com.example.someevent",
    "source" : "/mycontext/subcontext",
    "id" : "1234-1234-1234",
    "time" : "2018-04-05T03:56:24Z",
    "datacontenttype" : "application/xml",

    ... further attributes omitted ...

    "data" : {
        ... application data encoded in XML ...
    }
}

-----------------------------------------------
```

See more https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/bindings/kafka-protocol-binding.md

# Build

```
export KAFKA_USERNAME=BMX6E6JIHEPHFF2X
export KAFKA_SERVER=pkc-lzvrd.us-west4.gcp.confluent.cloud:9092
export KAFKA_PASSWORD=0kAkGlBft8rWK2RNnhH4VKd9QHjIF+8z/sjgr9wnVWSJ8r/Z0sVFmrmRGviVjoGU

cargo build

```

Then running command below to test it

```
./target/debug/kafka-transforms

```

# Running with docker

```
$ docker build . -t kafka-transforms:v1
$ docker run --rm kafka-transforms:v1                                                                         
listening on 0.0.0.0:3000 

```
# Test

To test sending a payload of Kafka messages, you can use the following command:

```
curl -X POST -H "Content-Type: application/json" -d '{ "luna": "0.4$", "btc": "100000$" }' 127.0.0.1:3000/send

{"message":"message sent"}

```
After executing the command, you can check the log inside the container.

```
Sending event: Event {
    attributes: V10(
        Attributes {
            id: "b1186fb9-ad14-4157-b8ae-2611fa013279",
            ty: "example.test",
            source: "http://localhost/",
            datacontenttype: Some(
                "application/json",
            ),
            dataschema: None,
            subject: None,
            time: Some(
                2023-10-13T06:20:21.068907199Z,
            ),
        },
    ),
    data: Some(
        Json(
            Object {
                "btc": String("100000$"),
                "luna": String("0.4$"),
            },
        ),
    ),
    extensions: {},
}


```

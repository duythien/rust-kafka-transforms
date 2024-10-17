# POC EDA Kafka

This is a demo service that uses EDA to consume messages from Kafka. In this case, we are demonstrating a simple scenario for a crypto trader. If the price of BTC is greater than 25000, then we will sell; otherwise, we will buy.

# Build

```
export KAFKA_USERNAME=BMX6E6JIHEPHFF2X
export KAFKA_SERVER=pkc-lzvrd.us-west4.gcp.confluent.cloud:9092
export KAFKA_PASSWORD=0kAkGlBft8rWK2RNnhH4VKd9QHjIF+8z/sjgr9wnVWSJ8r/Z0sVFmrmRGviVjoGU

cargo build

```

Then running command below to test it

```
./target/debug/kafka-poc

```

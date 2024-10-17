# This is the build stage for kafka transform
FROM --platform=$BUILDPLATFORM rust:1.71.1 as builder
ARG BUILDPLATFORM
WORKDIR /kafka

RUN apt-get update -y && apt-get install libwayland-dev libprotobuf-dev  clang libclang-dev protobuf-compiler cmake -y

COPY . /kafka
RUN cargo build --release

# This is the final stage: a very small image where we copy the Vban binary."
FROM ubuntu:22.04 as final
RUN apt-get update -y && apt-get install wget -y
RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.19_amd64.deb && dpkg -i libssl1.1_1.1.1f-1ubuntu2.19_amd64.deb
COPY --from=builder /kafka/target/release/kafka-transforms /usr/local/bin

RUN mkdir -p /root/.local/share && \
    mkdir /data && \
    ln -s /data /root/.local/share/kafka-transforms

VOLUME ["/data"]
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/kafka-transforms"]

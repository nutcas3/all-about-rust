use std::env::args;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use tokio::io::{AsyncBufReadExt,AsyncWriteExt,BufReader}
use uuid::{Uuid};

#[tokio::main]
async fn main() {
    let mut stdout = tokio::io::stdout();

    stdout.write(b"Welcome to Kafka!\n").await.unwrap();

}

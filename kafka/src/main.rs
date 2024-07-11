use std::env::args;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use tokio::io::{AsyncBufReadExt,AsyncWriteExt,BufReader}
use uuid::{Uuid};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();

    stdout.write(b"Welcome to Kafka!\n").await?;

    let name;
    loop {
        stdout.write(b"Please enter your name: ").await?;
        stdout.flush().await?;
 
        if let Some(s) = input_lines.next_line().await? {
            if s.is_empty() {
                continue;
            }
 
            name = s;
            break;
        }
    };
    let producer = create_producer(&args().skip(1).next()
    .unwrap_or("localhost:9092".to_string()))?;

stdout.write("-------------\n\n".as_bytes()).await.unwrap();

producer.send(FutureRecord::to("chat")
    .key(&name)
    .payload(b"has joined the chat"), Timeout::Never)
    .await
    .expect("Failed to produce");

let consumer = create_consumer(&args().skip(1).next()
    .unwrap_or("localhost:9092".to_string()))?;

consumer.subscribe(&["chat"])?;
}

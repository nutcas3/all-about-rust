use std::env::args;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use uuid::{Uuid};
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();
 
    stdout.write(b"Welcome to Kafka chat!\n").await?;
 
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
 
    loop {
        tokio::select! {
            message = consumer.recv() => {
                let message  = message.expect("Failed to read message").detach();
                let key = message.key().ok_or_else(|| "no key for message")?;
 
                if key == name.as_bytes() {
                    continue;
                }
 
                let payload = message.payload().ok_or_else(|| "no payload for message")?;
                stdout.write(b"\t").await?;
                stdout.write(key).await?;
                stdout.write(b": ").await?;
                stdout.write(payload).await?;
                stdout.write(b"\n").await?;
            }
            line = input_lines.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        producer.send(FutureRecord::to("chat")
                            .key(&name)
                            .payload(&line), Timeout::Never)
                            .await
                        .map_err(|(e, _)| format!("Failed to produce: {:?}", e))?;
                    }
                    _ => break,
                }
            }
        }
    }
 
    Ok(())
}
 
 
fn create_producer(bootstrap_server: &str) -> Result<FutureProducer, Box<dyn std::error::Error>> {
    Ok(ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("queue.buffering.max.ms", "0")
        .create()?)
}
 
fn create_consumer(bootstrap_server: &str) -> Result<StreamConsumer, Box<dyn std::error::Error>> {
    Ok(ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("group.id", format!("chat-{}", Uuid::new_v4()))
        .create()
        .expect("Failed to create client"))
}
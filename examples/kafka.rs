use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{Consumer, StreamConsumer, ConsumerContext};
use rdkafka::config::ClientConfig;
use tokio::runtime::Runtime;

// Function to produce messages to Kafka
fn produce_messages(bootstrap_servers: &str, topic: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("acks", "all")
        .create()
        .expect("Failed to create producer");

    // Produce a few messages
    for i in 0..5 {
        let record = FutureRecord::to(topic).payload(&format!("Message {}", i));
        producer.send(record, 0).expect("Failed to send message");
    }

    producer.flush(Duration::from_secs(1));
}

// Function to consume messages from Kafka
fn consume_messages(bootstrap_servers: &str, topic: &str) {
    let context = ConsumerContext::new();
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("group.id", "my-consumer-group")
        .set("enable.auto.commit", "false")
        .create_with_context(context)
        .expect("Failed to create consumer");

    consumer.subscribe(&[topic]).expect("Failed to subscribe to topic");

    // Consume messages in a loop
    loop {
        let message = consumer.poll(Duration::from_millis(100));
        match message {
            Some(Ok(msg)) => {
                println!("Received message: {}", msg.payload_view::<str>().unwrap());
                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
            Some(Err(err)) => {
                eprintln!("Error while consuming message: {:?}", err);
                break;
            }
            None => break,
        }
    }
}

fn main() {
    let bootstrap_servers = "localhost:9092";
    let topic = "my-topic";

    // Start a new thread to produce messages
    std::thread::spawn(move || {
        produce_messages(bootstrap_servers, topic);
    });

    // Create a Tokio runtime to run the consumer
    let mut runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        consume_messages(bootstrap_servers, topic);
    });
}

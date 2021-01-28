use clap::{App, Arg};
use log::info;
use flexi_logger::{Logger, colored_with_thread};

use tokio::time::{sleep, Duration};
use rdkafka::util::get_rdkafka_version;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;

#[tokio::main]
async fn main() {
    Logger::with_env()
        .format(colored_with_thread)
        .start()
        .unwrap();

    let matches = App::new("producer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .arg(Arg::new("prokers")
            .short('b')
            .long("brokers")
            .takes_value(true)
        )
        .arg(Arg::new("topic")
            .short('t')
            .long("topic")
            .takes_value(true)
        )
        .get_matches();
    
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topic = matches.value_of("topic").unwrap_or("test");
    let brokers = matches.value_of("brokers").unwrap_or("127.0.0.1:9094");
    
    info!("Going to produce");
    produce(topic, brokers).await;
    info!("Out of produce");
}

async fn produce(topic: &str, brokers: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .set("security.protocol", "plaintext")
        .create()
        .expect("Failed to create producer");

    let mut count: i32 = 0;

    loop {
        let payload = format!("message {}", count);
        let key = format!("key {}", count);

        info!("Sending message '{}'", count);

        let status = producer.send(
            FutureRecord::to(topic)
                .payload(&payload)
                .key(&key)
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
            Duration::from_secs(0)
        ).await;

        info!("Status '{:?}' received from message '{}'", status, count);

        count = count + 1;
        
        info!("Sleeping for 1 second");
        sleep(Duration::from_secs(5)).await;
        info!("Done sleeping");
    }
}
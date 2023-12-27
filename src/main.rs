use rumqttc::{self, Client, LastWill, MqttOptions, QoS};
use rumqttc::{AsyncClient, EventLoop};
use std::thread;
use std::time::Duration;
use tokio::{task, time};

use clap::Arg;
use clap::Parser;
use std::sync::Arc;

/// Simple program to run mqtt as server or client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "abcdefg")]
struct Args {
    /// role of the mqtt run
    #[arg(short, long)]
    role: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.role.as_str() {
        "pub" => {
            let mut mqttoptions = MqttOptions::new("test-2", "localhost", 1883);
            let will = LastWill::new("hello/world", "good bye", QoS::AtMostOnce, false);
            mqttoptions
                .set_keep_alive(Duration::from_secs(5))
                .set_last_will(will);

            let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
            let th = task::spawn(async move {
                publish(client).await;
            });
            // loop {
            //     let event = eventloop.poll().await;
            //     match &event {
            //         Ok(v) => {
            //             println!("Event = {v:?}");
            //         }
            //         Err(e) => {
            //             println!("Error = {e:?}");
            //         }
            //     }
            //     thread::sleep(Duration::from_millis(10));
            // }
            let th2 = task::spawn(async move {
                sub_dummy(eventloop).await;
            });

            thread::sleep(Duration::from_secs(1000));
        }
        _ => {
            println!("Error role");
        }
    }
}

async fn publish(mut client: AsyncClient) {
    // client.subscribe("hello/+/world", QoS::AtMostOnce).unwrap();
    time::sleep(Duration::from_secs(1)).await;

    for i in 0..10 {
        let payload = vec![1; i];
        let topic = format!("hello/{i}/world");
        let qos = QoS::AtLeastOnce;

        client
            .publish(topic.clone(), qos, true, payload)
            .await
            .unwrap();
        println!("+++ Publish i: {:?} topic:{}", i, topic);
        time::sleep(Duration::from_secs(1)).await;
    }
}

async fn sub_dummy(mut eventloop: EventLoop) {
    loop {
        println!("--- Sub");
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                println!("Event = {v:?}");
            }
            Err(e) => {
                println!("Error = {e:?}");
            }
        }
        time::sleep(Duration::from_secs(1)).await;
    }
}

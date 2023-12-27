use rumqttc::{self, Client, LastWill, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

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

fn main() {
    let args = Args::parse();
    match args.role.as_str() {
        "pub" => {
            let mut mqttoptions = MqttOptions::new("test-2", "localhost", 1883);
            let will = LastWill::new("hello/world", "good bye", QoS::AtMostOnce, false);
            mqttoptions
                .set_keep_alive(Duration::from_secs(5))
                .set_last_will(will);

            let (mut client, mut connection) = Client::new(mqttoptions, 10);
            let th = thread::spawn(move || publish(client));
            let th1 = thread::spawn(move || sub_dummy(connection));
            th1.join();
            th.join();
        }
        "sub" => {
            let mut mqttoptions = MqttOptions::new("test-3", "localhost", 1883);
            let will = LastWill::new("hello/world", "good bye", QoS::AtMostOnce, false);
            mqttoptions
                .set_keep_alive(Duration::from_secs(5))
                .set_last_will(will);

            let (mut client, mut connection) = Client::new(mqttoptions, 10);
            client.subscribe("hello/+/world", QoS::AtMostOnce).unwrap();
            let th1 = thread::spawn(move || sub(connection));
            th1.join();
        }
        "all" => {
            let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
            let will = LastWill::new("hello/world", "good bye", QoS::AtMostOnce, false);
            mqttoptions
                .set_keep_alive(Duration::from_secs(5))
                .set_last_will(will);

            let (mut client, connection) = Client::new(mqttoptions, 10);
            client.subscribe("hello/+/world", QoS::AtMostOnce).unwrap();
            let th = thread::spawn(move || publish(client));
            let th1 = thread::spawn(move || sub(connection));
            th.join();
            th1.join();
        }
        _ => {
            println!("Error role");
        }
    }
}

fn publish(mut client: Client) {
    for i in 0..10 {
        let payload = vec![1; i];
        let topic = format!("hello/{i}/world");
        let qos = QoS::AtLeastOnce;

        client.publish(topic.clone(), qos, true, payload).unwrap();
        println!("++Publish i: {:?} topic:{}", i, topic);
        thread::sleep(Duration::from_secs(6));
    }
}

fn sub(mut connection: rumqttc::Connection) {
    loop {
        if let Ok(notification) = connection.recv_timeout(Duration::from_millis(100)) {
            println!("-- Notification = {notification:?}");
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn sub_dummy(mut connection: rumqttc::Connection) {
    let _ = connection.recv_timeout(Duration::from_millis(100));

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

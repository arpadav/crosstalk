#![allow(dead_code)]

use std::thread;
use std::collections::HashMap;
use crosstalk::AsTopic;

// #[derive(Clone, Copy, PartialEq, Eq, Hash)] 
#[derive(AsTopic)] // required by crosstalk
enum TopicZoo {
    Topic1,
    Topic2,
    Topic3,
    Topic4,
    Topic5,
    Topic6,
}

#[derive(Clone)] // required by crosstalk
#[derive(PartialEq, Debug)]
struct Vehicle {
    make: String,
    model: String,
    color: Color,
    wheels: u8,
}

#[derive(Clone)] // required by crosstalk
#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Blue,
    Green
}

crosstalk::init! {
    TopicZoo::Topic1 => Vec<u32>,
    TopicZoo::Topic2 => String,
    TopicZoo::Topic3 => Vehicle,
    TopicZoo::Topic4 => HashMap<&str, Vec<Vehicle>>,
    TopicZoo::Topic5 => Color,
    // TopicZoo::Topic6 not included: defaults to String
}

fn main() {
    let mut node = crosstalk::UnboundedNode::<TopicZoo>::new();

    let (pub0_topic4, sub0_topic4) = node.pubsub(TopicZoo::Topic4).unwrap();
    let pub1_topic4 = pub0_topic4.clone();
    let sub1_topic4 = node.subscriber(TopicZoo::Topic4).unwrap();

    let pub0_topic5 = node.publisher::<Color>(TopicZoo::Topic5).unwrap();
    pub0_topic5.write(Color::Red);

    let message = HashMap::from([("my vehicles", vec![
        Vehicle { make: "Nissan".to_string(), model: "Rogue".to_string(), color: Color::Blue, wheels: 4 },
        Vehicle { make: "Toyota".to_string(), model: "Prius".to_string(), color: Color::Green, wheels: 4 },
    ])]);

    // pub0_topic4.write(message);

    thread::spawn(move || { pub0_topic4.write(message); });

    let received_0 = sub0_topic4.read();
    let received_1 = sub1_topic4.read();

    println!("{:?}", received_0);
    println!("{:?}", received_1);
    assert_eq!(received_0, received_1);

    node.delete_subscriber(sub1_topic4);

    let message2 = HashMap::from([("my vehicles 2", vec![
        Vehicle { make: "Honda".to_string(), model: "Civic".to_string(), color: Color::Red, wheels: 4 },
    ])]);

    pub1_topic4.write(message2);
    node.delete_publisher(pub1_topic4);

    thread::spawn(move || {
        let received_2 = sub0_topic4.read_blocking();
        println!("{:?}", received_2);
    }).join().unwrap();
}

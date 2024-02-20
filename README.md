# crosstalk

An extremely lightweight, topic-based (publisher / subscriber model), cross-thread, in-memory communication library using [flume](https://crates.io/crates/flume).

```rust
#![allow(dead_code)]

use std::thread;
use std::collections::HashMap;
use crosstalk::{ self, PubSub };

#[derive(Clone, Copy, PartialEq, Eq, Hash)] // required by crosstalk
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

fn main() {
    let mut node = crosstalk::unbounded! {
        TopicZoo::Topic1 => Vec<u32>,
        TopicZoo::Topic2 => String,
        TopicZoo::Topic3 => Vehicle,
        TopicZoo::Topic4 => HashMap<&str, Vec<Vehicle>>,
        TopicZoo::Topic5 => Color,
    };
    // TopicZoo::Topic6 not included: defaults to String

    let (pub0_topic4, sub0_topic4) = node
        .participant::<HashMap<&str, Vec<Vehicle>>>(TopicZoo::Topic4)
        .unwrap();
    let sub1_topic4 = node
        .subscriber::<HashMap<&str, Vec<Vehicle>>>(TopicZoo::Topic4)
        .unwrap();
    
    let message = HashMap::from([(
        "my vehicles",
        vec![
            Vehicle {
                make: "Nissan".to_string(),
                model: "Rogue".to_string(),
                color: Color::Blue,
                wheels: 4,
            },
            Vehicle {
                make: "Toyota".to_string(),
                model: "Prius".to_string(),
                color: Color::Green,
                wheels: 4,
            },
        ]
    )]);

    thread::spawn(move || { pub0_topic4.write(message); });

    let received_0 = sub0_topic4.read();
    let received_1 = sub1_topic4.read();

    println!("{:?}", received_0);
    println!("{:?}", received_1);
    assert_eq!(received_0, received_1);
}
```

## Why crosstalk?

All mpmc libraries focuses on a single FIFO channel, rather than broadcasting using topics; similar to any other pub/sub model. Crosstalk is used to dynamically create and destroy publishers and subscribers at runtime, across multiple threads. Crosstalk is a lightweight wrapper of [flume](https://crates.io/crates/flume), which does all the heavy lifting. Realistically crosstalk could wrap any mpmc library, but we decided on flume due to its simplicity, flexibility, and performance with handling unbounded channels.

## License

Crosstalk is released under the MIT license [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
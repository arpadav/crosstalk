# crosstalk

An extremely lightweight, topic-based (publisher / subscriber model), cross-thread, in-memory communication library using [flume](https://crates.io/crates/flume).

```rust
#![allow(dead_code)]

use std::thread;
use std::collections::HashMap;
use crosstalk::AsTopic;

#[derive(AsTopic)] // required for crosstalk topic
enum TopicZoo {
    Topic1,
    Topic2,
    Topic3,
    Topic4,
    Topic5,
    Topic6,
}

#[derive(Clone)] // required for crosstalk data
#[derive(PartialEq, Debug)]
struct Vehicle {
    make: String,
    model: String,
    color: Color,
    wheels: u8,
}

#[derive(Clone)] // required for crosstalk data
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
};
// TopicZoo::Topic6 not included: defaults to String

fn main() {
    let mut node = crosstalk::UnboundedNode::<TopicZoo>::new();

    let (pub0_topic5, sub0_topic5) = node
        .pubsub(TopicZoo::Topic5)
        .unwrap();
    let sub1_topic5 = node
        .subscriber(TopicZoo::Topic5)
        .unwrap();

    let message = Color::Red;

    thread::spawn(move || { pub0_topic5.write(message); });

    let received_0 = sub0_topic5.read();
    let received_1 = sub1_topic5.read();

    println!("{:?}", received_0);
    println!("{:?}", received_1);
    assert_eq!(received_0, received_1);
}
```

## Why crosstalk?

All mpmc libraries focuses on a single FIFO channel, rather than broadcasting using topics; similar to any other pub/sub model. Crosstalk is used to dynamically create and destroy publishers and subscribers at runtime, across multiple threads. Crosstalk is a lightweight wrapper of [flume](https://crates.io/crates/flume), which does all the heavy lifting. Realistically crosstalk could wrap any mpmc library, but we decided on flume due to its simplicity, flexibility, and performance with handling unbounded channels.

## License

Crosstalk is released under the MIT license [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
# crosstalk

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/crosstalk.svg)](https://crates.io/crates/crosstalk)
<!-- [![Latest Release](https://img.shields.io/github/v/release/arpadav/crosstalk)](https://github.com/arpadav/crosstalk) -->
<!-- [![Coverage Status](https://coveralls.io/repos/github/arpadav/crosstalk/badge.svg?branch=main)](https://coveralls.io/github/arpadav/crosstalk?branch=main) -->

A lightweight wrapper of [tokio](https://crates.io/crates/tokio)'s bounded broadcasting channels to enable topic-based (publisher/subscriber) paradigm of mpmc communication.

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
}
// TopicZoo::Topic6 not included: defaults to String

fn main() {
    let mut node = crosstalk::BoundedNode::<TopicZoo>::new(1024);

    let (pub0_topic5, mut sub0_topic5) = node
        .pubsub(TopicZoo::Topic5)
        .unwrap();
    let mut sub1_topic5 = node
        .subscriber(TopicZoo::Topic5)
        .unwrap();

    let message = Color::Red;

    thread::spawn(move || { pub0_topic5.write(message); });

    let received_0 = sub0_topic5.read_blocking();
    let received_1 = sub1_topic5.read_blocking();

    println!("{:?}", received_0);
    println!("{:?}", received_1);
    assert_eq!(received_0, received_1);
}
```

## Why crosstalk?

Most mpmc libraries focuses on a single FIFO channel, rather than broadcasting. [Tokio](https://crates.io/crates/tokio) is one of the only established mpmc / async libraries that supports broadcasting, so the motivation was to wrap `tokio`'s channels with a topic-based paradigm, similar to ROS, for ease of use. Crosstalk acts as a lightweight wrapper of `tokio::sync::broadcast`, correlating topic enums with datatypes and senders/receivers. Crosstalk can be used to dynamically create and destroy publishers and subscribers at runtime, across multiple threads. 

## License

Crosstalk is released under the MIT license [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
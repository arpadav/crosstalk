#![allow(non_snake_case)]
use std::thread::JoinHandle;
use crosstalk;
use criterion::{
    Criterion,
    black_box,
    criterion_main,
    criterion_group,
};

// --------------------------------------------------
// Example topics enum
// --------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TopicZoo {
    Topic1,
    // Topic2,
    // Topic3,
    // Topic4,
}

// // --------------------------------------------------
// // Example custom struct / datatype
// // --------------------------------------------------
// #[derive(Clone, Debug)]
// struct DetectorOutput {
//     bbox: (u32, u32, u32, u32),
//     score: f32,
//     class: i32,
// }

// --------------------------------------------------
// Constants
// --------------------------------------------------
const NUM_PUBLISHERS: usize = 10;
const NUM_SUBSCRIBERS: usize = 50;
const NUM_MESSAGES: usize = 1000;

/// For timing single message reception.
fn write<D>(p: &crosstalk::Publisher<D, TopicZoo>, msg: D) { p.write(msg); }
fn read<D>(s: &crosstalk::Subscriber<D, TopicZoo>) { s.read_blocking(); }
// debugging:
/*
fn read<D>(s: &crosstalk::Subscriber<D, TopicZoo>)
where
    D: std::fmt::Display,
{
    let start = black_box(std::time::Instant::now());
    let timeout = black_box(std::time::Duration::from_millis(10));
    let mut val = black_box(None);
    while let None = val {
        val = s.try_read();
        if black_box(std::time::Instant::now() > start + timeout) { break; }
    }
    black_box({
        match val {
            Some(val) => println!("{}", val),
            None => println!("None"),
        }
    });
}
*/


/// One message sent and received for topic with 1 publisher and 1 subscriber.
/// 
/// Returns when message is received from the single subscriber.
/// 
/// Timing: reception time.
fn transmit_once__1p1s<D>(p: crosstalk::Publisher<D, TopicZoo>, s: crosstalk::Subscriber<D, TopicZoo>, msg: D)
where
    D: std::fmt::Display + Sync + Send + 'static,
{
    let rthread = std::thread::spawn(move || { read(&s); });
    black_box(write(&p, msg));
    rthread.join().unwrap();
}

/// One message sent and received for topic with multiple publishers and 1 subscriber.
/// 
/// Returns when message is received from the single subscriber.
/// 
/// Timing: reception time.
fn transmit_once__mp1s<D>(ps: Vec<crosstalk::Publisher<D, TopicZoo>>, s: crosstalk::Subscriber<D, TopicZoo>, msg: D)
where
    D: std::fmt::Display + Sync + Send + 'static,
{
    let rthread = std::thread::spawn(move || { read(&s); });
    black_box(write(&ps[0], msg));
    rthread.join().unwrap();
}

/// One message sent and received for topic with 1 publisher and multiple subscribers.
/// 
/// Returns when message is received from all subscribers.
/// 
/// Timing: reception time.
fn transmit_once__1pms<D>(p: crosstalk::Publisher<D, TopicZoo>, ss: Vec<crosstalk::Subscriber<D, TopicZoo>>, msg: D)
where
    D: std::fmt::Display + Sync + Send + 'static,
{
    let mut rthreads: Vec<JoinHandle<()>> = black_box(Vec::new());
    for s in ss {
        let rthread = black_box(std::thread::spawn(move || { read(&s); }));
        black_box(rthreads.push(rthread));
    }
    black_box(write(&p, msg));
    rthreads.into_iter().for_each(|rthread| rthread.join().unwrap());
}


/// One message sent and received for topic with multiple publishers and multiple subscribers.
/// 
/// Returns when message is received from all subscribers.
/// 
/// Timing: reception time.
fn transmit_once__mpms<D>(ps: Vec<crosstalk::Publisher<D, TopicZoo>>, ss: Vec<crosstalk::Subscriber<D, TopicZoo>>, msg: D)
where
    D: std::fmt::Display + Sync + Send + 'static,
{
    let mut rthreads: Vec<JoinHandle<()>> = black_box(Vec::new());
    for s in ss {
        let rthread = black_box(std::thread::spawn(move || { read(&s); }));
        black_box(rthreads.push(rthread));
    }
    black_box(write(&ps[0], msg));
    rthreads.into_iter().for_each(|rthread| rthread.join().unwrap());
}

/// Drains all messages from a single subscriber.
/// 
/// Returns when all messages are received from the single subscriber.
fn drain__1s<D>(s: &crosstalk::Subscriber<D, TopicZoo>) {
    // s.read_blocking();
    s.read();
    while let Some(_) = s.try_read() {}
}

/// Drains all messages from multiple subscribers.
/// 
/// Returns when all messages are received from all the subscribers.
fn drain__ms<D>(ss: Vec<crosstalk::Subscriber<D, TopicZoo>>)
where
    D: Sync + Send + 'static,
{
    let mut rthreads = black_box(Vec::new());
    for s in ss {
        let rthread = black_box(std::thread::spawn(move || { drain__1s(&s); }));
        black_box(rthreads.push(rthread));
    }
    rthreads.into_iter().for_each(|rthread| rthread.join().unwrap());
}

/// Multiple messages sent and received for topic with 1 publisher and 1 subscriber.
/// 
/// Returns when all messages are received from the single subscriber.
/// 
/// Timing: reception time.
fn transmit_many__1p1s<D>(p: &crosstalk::Publisher<D, TopicZoo>, s: &crosstalk::Subscriber<D, TopicZoo>, msg: Vec<D>) {
    black_box(push__1p(p, msg));
    drain__1s(s);
}

/// Multiple messages sent and received for topic with multiple publishers and 1 subscriber.
/// 
/// Returns when all messages are received from the single subscriber.
/// 
/// Timing: reception time.
fn transmit_many__mp1s<D>(ps: Vec<crosstalk::Publisher<D, TopicZoo>>, s: &crosstalk::Subscriber<D, TopicZoo>, msg: Vec<D>)
where 
    D: Clone + Sync + Send + 'static,
{
    black_box(push__mp(ps, msg));
    drain__1s(s);
}

/// Multiple messages sent and received for topic with 1 publisher and multiple subscribers.
/// 
/// Returns when all messages are received from all subscribers.
/// 
/// Timing: reception time.
fn transmit_many__1pms<D>(p: &crosstalk::Publisher<D, TopicZoo>, ss: Vec<crosstalk::Subscriber<D, TopicZoo>>, msg: Vec<D>)
where
    D: Sync + Send + 'static,
{
    black_box(push__1p(p, msg));
    drain__ms(ss);
}

/// Multiple messages sent and received for topic with multiple publishers and multiple subscribers.
/// 
/// Returns when all messages are received from all subscribers.
/// 
/// Timing: reception time.
fn transmit_many__mpms<D>(ps: Vec<crosstalk::Publisher<D, TopicZoo>>, ss: Vec<crosstalk::Subscriber<D, TopicZoo>>, msg: Vec<D>)
where
    D: Clone + Sync + Send + 'static,
{
    black_box(push__mp(ps, msg));
    drain__ms(ss);
}

/// Multiple messages sent using one publisher.
/// 
/// Returns when all messages are sent.
/// 
/// Timing: transmission time.
fn push__1p<D>(p: &crosstalk::Publisher<D, TopicZoo>, msg: Vec<D>) {
    msg.into_iter().for_each(|m| p.write(m));
}


/// Multiple messages sent using multiple publishers.
/// 
/// Returns when all messages are sent.
/// 
/// Timing: transmission time.
fn push__mp<D>(ps: Vec<crosstalk::Publisher<D, TopicZoo>>, msg: Vec<D>)
where
    D: Clone + Sync + Send + 'static,
{
    let mut pthreads = black_box(Vec::new());
    for p in ps {
        let msg_clone = black_box(msg.clone());
        let pthread = black_box(std::thread::spawn(move || { push__1p(&p, msg_clone); }));
        black_box(pthreads.push(pthread));
    }
    pthreads.into_iter().for_each(|p| p.join().unwrap());
}

crosstalk::init! {
    TopicZoo::Topic1 => String,
    // TopicZoo::Topic2 => String,
    // TopicZoo::Topic3 => String,
    // TopicZoo::Topic4 => String,
}

fn unode__only_string() -> crosstalk::UnboundedNode<TopicZoo> {
    crosstalk::UnboundedNode::<TopicZoo>::new()
}


fn benchmark_t1_1p0s__only_string(c: &mut Criterion) {
    c.bench_function("t1_1p0s__only_string", |b| {
        let mut node = unode__only_string();
        let msg = black_box("Hello World".to_string());
        let p = node.publisher::<String>(TopicZoo::Topic1).unwrap();
        b.iter(|| {
            write(&p, black_box(msg.clone()))
        });
        
    });
}


fn benchmark_t1_1p1s__only_string(c: &mut Criterion) {
    c.bench_function("t1_1p1s__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msg = black_box("Hello World".to_string());
            let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
            let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
            transmit_once__1p1s(p, s, black_box(msg.clone()));
        });
        
    });
}


fn benchmark_t1_1pms__only_string(c: &mut Criterion) {
    c.bench_function("t1_1pms__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msg = black_box("Hello World".to_string());
            let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
            let mut ss = black_box(Vec::new());
            for _ in 0..NUM_SUBSCRIBERS {
                let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
                black_box(ss.push(s));
            }
            transmit_once__1pms(p, ss, black_box(msg.clone()));
        });
        
    });
}


fn benchmark_t1_mp1s__only_string(c: &mut Criterion) {
    c.bench_function("t1_mp1s__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msg = black_box("Hello World".to_string());
            let mut ps = black_box(Vec::new());
            for _ in 0..NUM_PUBLISHERS {
                let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
                black_box(ps.push(p));
            }
            let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
            transmit_once__mp1s(ps, s, black_box(msg.clone()));
        });        
    });
}


fn benchmark_t1_mpms__only_string(c: &mut Criterion) {
    c.bench_function("t1_mpms__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msg = black_box("Hello World".to_string());
            let mut ps = black_box(Vec::new());
            for _ in 0..NUM_PUBLISHERS {
                let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
                black_box(ps.push(p));
            }
            let mut ss = black_box(Vec::new());
            for _ in 0..NUM_SUBSCRIBERS {
                let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
                black_box(ss.push(s));
            }
            transmit_once__mpms(ps, ss, black_box(msg.clone()));
        });        
    });
}


fn benchmark_tm_1p0s__only_string(c: &mut Criterion) {
    c.bench_function("tm_1p0s__only_string", |b| {
        let mut node = unode__only_string();
        let msgs = black_box(vec!["Hello World".to_string(); NUM_MESSAGES]);
        let p = node.publisher::<String>(TopicZoo::Topic1).unwrap();
        b.iter(|| {
            push__1p(&p, black_box(msgs.clone()));
        });
    });
}


fn benchmark_tm_1p1s__only_string(c: &mut Criterion) {
    c.bench_function("tm_1p1s__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msgs = black_box(vec!["Hello World".to_string(); NUM_MESSAGES]);
            let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
            let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
            transmit_many__1p1s(&p, &s, black_box(msgs.clone()));
        });
    });
}


fn benchmark_tm_1pms__only_string(c: &mut Criterion) {
    c.bench_function("tm_1pms__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msgs = black_box(vec!["Hello World".to_string(); NUM_MESSAGES]);
            let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
            let mut ss = black_box(Vec::new());
            for _ in 0..NUM_SUBSCRIBERS {
                let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
                black_box(ss.push(s));
            }
            transmit_many__1pms(&p, ss, black_box(msgs.clone()));
        });
    });
}


fn benchmark_tm_mp1s__only_string(c: &mut Criterion) {
    c.bench_function("tm_mp1s__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msgs = black_box(vec!["Hello World".to_string(); NUM_MESSAGES]);
            let mut ps = black_box(Vec::new());
            for _ in 0..NUM_PUBLISHERS {
                let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
                black_box(ps.push(p));
            }
            let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
            transmit_many__mp1s(ps, &s, black_box(msgs.clone()));
        });
    });
}


fn benchmark_tm_mpms__only_string(c: &mut Criterion) {
    c.bench_function("tm_mpms__only_string", |b| {
        b.iter(|| {
            let mut node = black_box(unode__only_string());
            let msgs = black_box(vec!["Hello World".to_string(); NUM_MESSAGES]);
            let mut ps = black_box(Vec::new());
            for _ in 0..NUM_PUBLISHERS {
                let p = black_box(node.publisher::<String>(TopicZoo::Topic1).unwrap());
                black_box(ps.push(p));
            }
            let mut ss = black_box(Vec::new());
            for _ in 0..NUM_SUBSCRIBERS {
                let s = black_box(node.subscriber::<String>(TopicZoo::Topic1).unwrap());
                black_box(ss.push(s));
            }
            transmit_many__mpms(ps, ss, black_box(msgs.clone()));
        });
    });
}

// --------------------------------------------------
// Benchmarks
// --------------------------------------------------
criterion_group!(
    benches,
    benchmark_t1_1p0s__only_string,
    benchmark_t1_1p1s__only_string,
    benchmark_t1_1pms__only_string,
    benchmark_t1_mp1s__only_string,
    benchmark_t1_mpms__only_string,
    benchmark_tm_1p0s__only_string,
    benchmark_tm_1p1s__only_string,
    benchmark_tm_1pms__only_string,
    benchmark_tm_mp1s__only_string,
    benchmark_tm_mpms__only_string,
);
criterion_main!(benches);

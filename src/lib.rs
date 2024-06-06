//! # crosstalk
//! 
//! A lightweight wrapper of [tokio](https://crates.io/crates/tokio)'s bounded broadcasting channels to enable topic-based (publisher/subscriber) paradigm of mpmc communication.
//! 
//! ```rust
//! #![allow(dead_code)]
//! 
//! use std::thread;
//! use std::collections::HashMap;
//! use crosstalk::AsTopic;
//! 
//! #[derive(AsTopic)] // required for crosstalk topic
//! enum TopicZoo {
//!     Topic1,
//!     Topic2,
//!     Topic3,
//!     Topic4,
//!     Topic5,
//!     Topic6,
//! }
//! 
//! #[derive(Clone)] // required for crosstalk data
//! #[derive(PartialEq, Debug)]
//! struct Vehicle {
//!     make: String,
//!     model: String,
//!     color: Color,
//!     wheels: u8,
//! }
//! 
//! #[derive(Clone)] // required for crosstalk data
//! #[derive(PartialEq, Debug)]
//! enum Color {
//!     Red,
//!     Blue,
//!     Green
//! }
//! 
//! crosstalk::init! {
//!     TopicZoo::Topic1 => Vec<u32>,
//!     TopicZoo::Topic2 => String,
//!     TopicZoo::Topic3 => Vehicle,
//!     TopicZoo::Topic4 => HashMap<&str, Vec<Vehicle>>,
//!     TopicZoo::Topic5 => Color,
//! }
//! // TopicZoo::Topic6 not included: defaults to String
//! 
//! fn main() {
//!     let mut node = crosstalk::BoundedNode::<TopicZoo>::new(1024);
//! 
//!     let (pub0_topic5, mut sub0_topic5) = node
//!         .pubsub(TopicZoo::Topic5)
//!         .unwrap();
//!     let mut sub1_topic5 = node
//!         .subscriber(TopicZoo::Topic5)
//!         .unwrap();
//! 
//!     let message = Color::Red;
//! 
//!     thread::spawn(move || { pub0_topic5.write(message); });
//! 
//!     let received_0 = sub0_topic5.read_blocking();
//!     let received_1 = sub1_topic5.read_blocking();
//! 
//!     println!("{:?}", received_0);
//!     println!("{:?}", received_1);
//!     assert_eq!(received_0, received_1);
//! }
//! ```
//! 
//! ## Why crosstalk?
//! 
//! Most mpmc libraries focuses on a single FIFO channel, rather than broadcasting. [Tokio](https://crates.io/crates/tokio) is one of the only established mpmc / async libraries that supports broadcasting, so the motivation was to wrap `tokio`'s channels with a topic-based paradigm, similar to ROS, for ease of use. Crosstalk acts as a lightweight wrapper of `tokio::sync::broadcast`, correlating topic enums with datatypes and senders/receivers. Crosstalk can be used to dynamically create and destroy publishers and subscribers at runtime, across multiple threads. 
//! 
//! ## License
//! 
//! Crosstalk is released under the MIT license [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
// --------------------------------------------------
// external
// --------------------------------------------------
use std::{
    sync::{
        Arc,
        Mutex,
    },
    any::Any,
};
use core::hash::Hash;
use hashbrown::HashMap;

// --------------------------------------------------
// internal
// --------------------------------------------------
pub mod external;
pub use crosstalk_macros::init;
pub use crosstalk_macros::AsTopic;

#[derive(Clone)]
/// A [`BoundedNode`] is a node to spawn publishers and
/// subscribers on, where the size of each buffer is
/// fixed.
/// 
/// # Attributes
/// 
/// * `node` - the node to spawn publishers and subscribers on
/// * `size` - the size of each buffer
/// 
/// # Type Parameters
/// 
/// * `T` - the topic enum name
/// 
/// # Examples
/// 
/// ```
/// use crosstalk::AsTopic;
/// 
/// #[derive(AsTopic)]
/// enum House {
///     Bedroom,
///     LivingRoom,
///     Kitchen,
///     Bathroom,
/// }
/// 
/// crosstalk::init! {
///     House::Bedroom => String,
///     House::LivingRoom => String,
///     House::Kitchen => Vec<f32>,
///     House::Bathroom => u8,
/// }
/// 
/// fn main() {
///     let mut node = crosstalk::BoundedNode::<House>::new(10);
///     let (pub0, mut sub0) = node.pubsub(House::Bedroom).unwrap();
///     let (pub1, mut sub1) = node.pubsub(House::Bedroom).unwrap();
///     
///     pub0.write("Hello".to_string());
///     pub0.write("World".to_string());
///     pub1.write("Foo".to_string());
///     pub1.write("Bar".to_string());
///     
///     assert_eq!(sub1.try_read().unwrap(), "Hello");
///     assert_eq!(sub1.try_read().unwrap(), "World");
///     assert_eq!(sub1.try_read().unwrap(), "Foo");
///     assert_eq!(sub1.try_read().unwrap(), "Bar");
///     
///     assert_eq!(sub0.try_read().unwrap(), "Hello");
///     assert_eq!(sub0.try_read().unwrap(), "World");
///     assert_eq!(sub0.try_read().unwrap(), "Foo");
///     assert_eq!(sub0.try_read().unwrap(), "Bar");
/// }
/// ```
pub struct BoundedNode<T> {
    pub node: Arc<Mutex<ImplementedBoundedNode<T>>>,
    pub size: usize,
}
/// Implements [`BoundedNode`]
/// 
/// This holds an [`Arc<Mutex<ImplementedBoundedNode<T>>>`], which
/// references the true (private) node that implements the [`AsTopic`] trait.
impl<T> BoundedNode<T> 
where
    T: CrosstalkTopic,
    ImplementedBoundedNode<T>: CrosstalkPubSub<T>,
{
    #[inline]
    /// Creates a new [`BoundedNode`]
    /// 
    /// # Arguments
    /// 
    /// * `size` - the size of each buffer
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let node = crosstalk::BoundedNode::<House>::new(10);
    ///     let moved_node = node.clone();
    ///     std::thread::spawn(move || another_thread(moved_node));
    ///     assert_eq!(node.size, 10);
    /// }
    /// 
    /// fn another_thread(mut node: crosstalk::BoundedNode<House>) {
    ///     assert_eq!(node.size, 10);
    /// }
    /// ```
    pub fn new(size: usize) -> Self {
        Self {
            node: Arc::new(Mutex::new(ImplementedBoundedNode::<T>::new(size.clone()))),
            size,
        }
    }

    #[inline]
    /// Creates a new publisher for the given topic `T`
    /// 
    /// # Arguments
    /// 
    /// * `topic` - the topic to create a publisher for
    /// 
    /// # Returns
    /// 
    /// A publisher for the topic `T`
    /// 
    /// # Panics
    /// 
    /// Will panic if the datatype of the topic `T` initialized
    /// using [`crosstalk_macros::init`] does not match the topic `T`.
    /// 
    /// Note that the ***linter will not catch this error until runtime***!
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     assert!(node.publisher::<String>(House::Bedroom).is_ok());
    /// }
    /// ```
    pub fn publisher<D: 'static>(&mut self, topic: T) -> Result<Publisher<D, T>, Box<dyn std::error::Error>> {
        let mut n = self.node.lock().unwrap();
        n.publisher(topic)
    }

    #[inline]
    /// Creates a new subscriber for the given topic `T`
    /// 
    /// # Arguments
    /// 
    /// * `topic` - the topic to create a subscriber for
    /// 
    /// # Returns
    /// 
    /// A subscriber for the topic `T`
    /// 
    /// # Panics
    /// 
    /// Will panic if the datatype of the topic `T` initialized
    /// using [`crosstalk_macros::init`] does not match the topic `T`.
    /// 
    /// Note that the ***linter will not catch this error until runtime***!
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     assert!(node.subscriber::<String>(House::Bedroom).is_ok());
    /// }
    /// ```
    pub fn subscriber<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<Subscriber<D, T>, Box<dyn std::error::Error>> {
        let mut n = self.node.lock().unwrap();
        n.subscriber(topic)
    }

    #[inline]
    /// Creates a new publisher and subscriber for the given topic `T`
    /// 
    /// # Arguments
    /// 
    /// * `topic` - the topic to create a publisher and subscriber for
    /// 
    /// # Returns
    /// 
    /// A publisher and subscriber for the topic `T`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.try_read().unwrap(), "hello");
    /// }
    /// ```
    /// 
    pub fn pubsub<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), Box<dyn std::error::Error>> {
        let mut n = self.node.lock().unwrap();
        n.pubsub(topic)
    }

    #[inline]
    /// Deletes a publisher
    /// 
    /// # Arguments
    /// 
    /// * `publisher` - the publisher to delete
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    ///
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    ///
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    ///
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let publisher = node.publisher::<String>(House::Bedroom).unwrap();
    ///     node.delete_publisher(publisher);
    ///     assert!(true);
    /// }
    /// ```
    pub fn delete_publisher<D: 'static>(&mut self, _publisher: Publisher<D, T>) {
        let mut n = self.node.lock().unwrap();
        n.delete_publisher(_publisher)
    }

    #[inline]
    /// Deletes a subscriber
    /// 
    /// # Arguments
    /// 
    /// * `subscriber` - the subscriber to delete
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    ///
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    ///
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    ///
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let subscriber = node.subscriber::<String>(House::Bedroom).unwrap();
    ///     node.delete_subscriber(subscriber);
    ///     assert!(true);
    /// }
    /// ```
    pub fn delete_subscriber<D: Clone + Send + 'static>(&mut self, subscriber: Subscriber<D, T>) {
        let mut n = self.node.lock().unwrap();
        n.delete_subscriber(subscriber)
    }
}


/// The inner implementation of the node,
/// which implements the [`AsTopic`] trait
/// 
/// This is auto-generated by the [`crosstalk_macros::init!`] macro
/// 
/// # Attributes
/// 
/// * `senders` - the senders of the node
/// * `runtimes` - the [`tokio`] runtimes of the node
/// * `create_runtimes` - whether to create runtimes
/// * `size` - the size of each buffer
pub struct ImplementedBoundedNode<T> {
    pub senders: HashMap<T, Box<dyn Any + 'static>>,
    pub runtimes: HashMap<T, Arc<tokio::runtime::Runtime>>,
    pub create_runtimes: bool,
    pub size: usize,
}
/// Implements [`Send`](std::marker::Send) and [`Sync`](std::marker::Sync) for [`ImplementedBoundedNode`].
unsafe impl<T> Send for ImplementedBoundedNode<T> {}
unsafe impl<T> Sync for ImplementedBoundedNode<T> {}
/// Implements [`ImplementedBoundedNode`]
impl<T> ImplementedBoundedNode<T>
where
    T: CrosstalkTopic,
{
    /// See [`BoundedNode::new`]
    /// 
    /// # Arguments
    /// 
    /// * `size` - the size of each buffer
    pub fn new(size: usize) -> Self {
        Self {
            senders: HashMap::new(),
            runtimes: HashMap::new(),
            create_runtimes: tokio::runtime::Handle::try_current().is_err(),
            size: size,
        }
    }
}

#[derive(Clone)]
/// A `crosstalk` [`Publisher`]
/// 
/// # Attributes
/// 
/// * `topic` - the topic of the publisher
/// * `buf` - the buffer which broadcasts the data
/// 
/// # Type Parameters
/// 
/// * `T` - the topic of the publisher
/// * `D` - the data type of the publisher
/// 
/// This is not meant to be used directly, please
/// use the [`crosstalk_macros::init!`] macro instead
/// and produce a [`Publisher`] with [`BoundedNode::publisher`]
/// or [`BoundedNode::pubsub`]
pub struct Publisher<D, T> {
    pub topic: T,
    buf: tokio::sync::broadcast::Sender<D>,
}
/// Implements [`Publisher`]
impl<D, T> Publisher<D, T> {
    #[inline]
    /// See [`BoundedNode::publisher`]
    pub fn new(buf: tokio::sync::broadcast::Sender<D>, topic: T) -> Self {
        Self { buf, topic }
    }

    #[inline]
    /// Publishes data to a topic, broadcasting it to all subscribers
    /// 
    /// # Arguments
    /// 
    /// * `sample` - the sample to publish
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     publisher.write("hello".to_string());
    ///     std::thread::spawn(move || {
    ///         assert_eq!(subscriber.try_read().unwrap(), "hello");
    ///     });
    /// }
    /// ```
    pub fn write(&self, sample: D) {
        let _ = self.buf.send(sample);
    }
}

/// A `crosstalk` [`Subscriber`]
/// 
/// # Attributes
/// 
/// * `topic` - the topic of the subscriber
/// * `rcvr` - the receiver of the subscriber
/// * `sndr` - the sender for the topic. This is used to spawn multiple receivers upon [`Subscriber::clone`]
/// * `rt` - the [`tokio`] runtime of the subscriber
/// 
/// # Type Parameters
/// 
/// * `T` - the topic of the subscriber
/// * `D` - the data type of the subscriber
/// 
/// This is not meant to be used directly, please
/// use the [`crosstalk_macros::init!`] macro instead
/// and produce a [`Subscriber`] with [`BoundedNode::subscriber`]
/// or [`BoundedNode::pubsub`]
pub struct Subscriber<D, T> {
    pub topic: T,
    rcvr: Receiver<D>,
    sndr: Arc<tokio::sync::broadcast::Sender<D>>,
    rt: Option<Arc<tokio::runtime::Runtime>>,
}
/// Implements [`Subscriber`]
impl<D: Clone, T: Clone> Subscriber<D, T> {
    #[inline]
    /// See [`BoundedNode::subscriber`]
    pub fn new(
        topic: T,
        rcvr: Option<tokio::sync::broadcast::Receiver<D>>,
        sndr: Arc<tokio::sync::broadcast::Sender<D>>,
        rt: Option<Arc<tokio::runtime::Runtime>>,
    ) -> Self {
        let rcvr = rcvr.unwrap_or(sndr.subscribe());
        match rt {
            Some(rt) => Self {
                topic: topic,
                rcvr: Receiver::new(
                    rcvr,
                    Some(rt.clone()),
                ),
                sndr: sndr.clone(),
                rt: Some(rt),
            },
            None => Self {
                topic: topic,
                rcvr: Receiver::new(
                    rcvr,
                    None,
                ),
                sndr: sndr.clone(),
                rt: None,
            },
        }
    }

    #[inline]
    /// Clones a [`Subscriber`]
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     let mut subscriber_2 = subscriber.clone();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.try_read().unwrap(), "hello");
    ///     assert_eq!(subscriber_2.try_read().unwrap(), "hello");
    /// }
    /// ```
    pub fn clone(&self) -> Self {
        Self {
            topic: self.topic.clone(),
            rcvr: Receiver::new(
                self.sndr.subscribe(),
                self.rt.clone(),
            ),
            sndr: self.sndr.clone(),
            rt: self.rt.clone(),
        }
    }

    /// Asynchronous blocking read from the [`tokio::sync::broadcast::Receiver`]
    /// 
    /// The sequential equivalent to this function is [`Subscriber::read_blocking`]
    /// which can be used outside of an asynchronous context
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.read().await, Some("hello".to_string()));
    /// }
    /// ```
    pub async fn read(&mut self) -> Option<D> {
        self.rcvr.read().await
    }
    
    #[inline]
    /// Non-blocking read from [`tokio::sync::broadcast::Receiver`]
    /// Upon immediate failure, [`None`] will be returned
    /// 
    /// Difference between this function and [`Subscriber::try_read_raw`]
    /// is that this function continuously loops upon [`tokio`] error of
    /// [`tokio::sync::broadcast::error::TryRecvError::Lagged`], looping
    /// until a valid message is received OR the buffer is determined to be
    /// empty
    /// 
    /// [`Subscriber::try_read_raw`] will return [`None`] upon
    /// [`tokio::sync::broadcast::error::TryRecvError::Lagged`], which
    /// can cause some unexpected behavior
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.try_read(), Some(String::from("hello")));
    ///     assert_eq!(subscriber.try_read(), None);
    /// }
    /// ```
    pub fn try_read(&mut self) -> Option<D> {
        self.rcvr.try_read()
    }

    #[inline]
    /// Non-blocking read from [`tokio::sync::broadcast::Receiver`], returning
    /// [`None`] if there are no messages available or if 
    /// [`tokio::sync::broadcast::error::TryRecvError::Lagged`] occurs.
    /// 
    /// This function can cause some unexpected behavior. It is recommended
    /// to use [`Subscriber::try_read`] instead.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.try_read_raw(), Some(String::from("hello")));
    ///     assert_eq!(subscriber.try_read_raw(), None);
    /// }
    /// ```
    pub fn try_read_raw(&mut self) -> Option<D> {
        self.rcvr.try_read_raw()
    }
    
    #[inline]
    /// Sequential blocking read from [`tokio::sync::broadcast::Receiver`]
    /// 
    /// The asynchronous equivalent to this function is [`Subscriber::read`], 
    /// which must be used in an asynchronous context with `.await`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.read_blocking(), Some("hello".to_string()));
    /// }
    /// ```
    pub fn read_blocking(&mut self) -> Option<D> {
        self.rcvr.read_blocking()
    }
    
    #[inline]
    /// Asynchronous non-blocking read from [`tokio::sync::broadcast::Receiver`]
    /// with a given timeout. After the timeout if there are no messages,
    /// returns [`None`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::AsTopic;
    /// 
    /// #[derive(AsTopic)]
    /// enum House {
    ///     Bedroom,
    ///     LivingRoom,
    ///     Kitchen,
    ///     Bathroom,
    /// }
    /// 
    /// crosstalk::init! {
    ///     House::Bedroom => String,
    ///     House::LivingRoom => String,
    ///     House::Kitchen => Vec<f32>,
    ///     House::Bathroom => u8,
    /// }
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.read_timeout(std::time::Duration::from_millis(100)).await, Some("hello".to_string()));
    /// }
    /// ```
    pub async fn read_timeout(&mut self, timeout: std::time::Duration) -> Option<D> {
        self.rcvr.read_timeout(timeout).await
    }

    #[inline]
    #[deprecated]
    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.rcvr.set_timeout(timeout);
    }
}


/// Receiver
/// 
/// Define a receiver for subscribing messages
/// 
/// Reads from tokio::sync::broadcast::Receiver
pub struct Receiver<D> {
    buf: tokio::sync::broadcast::Receiver<D>,
    rt: Option<Arc<tokio::runtime::Runtime>>,
    timeout: std::time::Duration,
}
impl<D> Receiver<D>
where
    D: Clone
{
    #[inline]
    pub fn new(
        buf: tokio::sync::broadcast::Receiver<D>,
        rt: Option<Arc<tokio::runtime::Runtime>>,
    ) -> Self {
        let timeout = std::time::Duration::from_millis(10);
        Self {
            buf: buf,
            rt: rt,
            timeout: timeout,
        }
    }

    #[inline]
    /// Reads from [`tokio::sync::broadcast::Receiver`] after
    /// entering a [`tokio::runtime::Runtime`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::read`]
    pub async fn read(&mut self) -> Option<D> {
        let _guard = match self.rt {
            Some(ref rt) => Some(rt.enter()),
            None => None,
        };
        loop {
            match self.buf.recv().await {
                Ok(res) => return Some(res),
                Err(e) => match e {
                    tokio::sync::broadcast::error::RecvError::Lagged(_) => { continue; }
                    _ => {
                        // TODO: improve this error
                        log::error!("Some RecvError: {:?}", e);
                        return None
                    }
                }
            }
        }
    }    

    #[inline]
    /// Reads from [`tokio::sync::broadcast::Receiver`] after
    /// entering a [`tokio::runtime::Runtime`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::try_read`]
    pub fn try_read(&mut self) -> Option<D> {
        let _guard = match self.rt {
            Some(ref rt) => Some(rt.enter()),
            None => None,
        };
        loop {
            match self.buf.try_recv() {
                Ok(d) => return Some(d),
                Err(e) => {
                    match e {
                        tokio::sync::broadcast::error::TryRecvError::Lagged(_) => { continue; },
                        _ => return None,
                    }
                },
            }
        } 
    }
    
    #[inline]
    /// Reads from [`tokio::sync::broadcast::Receiver`] after
    /// entering a [`tokio::runtime::Runtime`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::try_read_raw`]
    pub fn try_read_raw(&mut self) -> Option<D> {
        let _guard = match self.rt {
            Some(ref rt) => Some(rt.enter()),
            None => None,
        };
        match self.buf.try_recv() {
            Ok(d) => Some(d),
            Err(_) => None,
        }
    }
    
    #[inline]
    /// Reads from [`tokio::sync::broadcast::Receiver`] after
    /// entering a [`tokio::runtime::Runtime`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::read_blocking`]
    pub fn read_blocking(&mut self) -> Option<D> {
        let _guard = match self.rt {
            Some(ref rt) => Some(rt.enter()),
            None => None,
        };
        loop {
            match self.buf.blocking_recv() {
                Ok(res) => return Some(res),
                Err(e) => match e {
                    tokio::sync::broadcast::error::RecvError::Lagged(_) => { continue; }
                    _ => {
                        // TODO: improve this error
                        log::error!("Some RecvError: {:?}", e);
                        return None
                    }
                }
            }
        }
    }
    
    #[inline]
    /// Reads from [`tokio::sync::broadcast::Receiver`] after
    /// entering a [`tokio::runtime::Runtime`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::read_timeout`]
    pub async fn read_timeout(&mut self, timeout: std::time::Duration) -> Option<D> {
        let _guard = match self.rt {
            Some(ref rt) => Some(rt.enter()),
            None => None,
        };
        match tokio::runtime::Handle::try_current() {
            Ok(_) => {
                match tokio::time::timeout(timeout, self.buf.recv()).await {
                    Ok(res) => {
                        match res {
                            Ok(res) => Some(res),
                            Err(e) => {
                                // TODO: improve this error
                                log::error!("Some RecvError: {:?}", e);
                                None
                            }
                        }
                    },
                    Err(e) => {
                        // TODO: improve this error
                        log::error!("Timeout error occurred: {:?}", e);
                        None
                    }
                }
            },
            Err(e) => {
                // TODO: improve this error
                log::error!("Tokio runtime is said to be running, but can not find current handle: {:?}", e);
                None
            },
        }
    }
    
    #[inline]
    /// This function is no longer in use
    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }
}

#[derive(Debug)]
/// `crosstalk` errors
pub enum Error {
    PublisherMismatch(String, String),
    SubscriberMismatch(String, String),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PublisherMismatch(input, output) => write!(f, "Publisher type mismatch: {} (cast) != {} (expected)", input, output),
            Error::SubscriberMismatch(input, output) => write!(f, "Subscriber type mismatch: {} (cast) != {} (expected)", input, output),
        }
    }
}

/// A trait to define a [`CrosstalkTopic`]
pub trait CrosstalkTopic: Eq + Hash + Copy + Clone + PartialEq {}
/// A trait to define a [`CrosstalkPubSub`]
/// 
/// This is used to implement the [`CrosstalkPubSub`] trait
/// using the [`crosstalk_macros::init`] macro
/// for the [`ImplementedBoundedNode`] struct
/// 
/// This is not meant to be used directly
pub trait CrosstalkPubSub<T> {
    fn publisher<D: 'static>(&mut self, topic: T) -> Result<Publisher<D, T>, Box<dyn std::error::Error>>;
    fn subscriber<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<Subscriber<D, T>, Box<dyn std::error::Error>>;
    fn pubsub<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), Box<dyn std::error::Error>>;
    fn delete_publisher<D: 'static>(&mut self, _publisher: Publisher<D, T>);
    fn delete_subscriber<D: Clone + Send + 'static>(&mut self, subscriber: Subscriber<D, T>);
}

#[inline]
/// Downcasts a [`Box`] into a type `T`
/// 
/// # Arguments
/// 
/// * `buf` - the [`Box`] to downcast
/// 
/// # Examples
/// 
/// ```
/// use crosstalk::downcast;
/// 
/// let mut buf = Box::new(5) as Box<dyn std::any::Any + 'static>;
/// assert_eq!(downcast::<i32>(buf).unwrap(), 5);
/// ```
pub fn downcast<T>(buf: Box<dyn Any + 'static>) -> Result<T, Box<dyn Any>>
where
    T: 'static,
{
    match buf.downcast::<T>() {
        Ok(t) => Ok(*t),
        Err(e) => Err(e),
    }
}

/// Converts a [`Result`]<_, `E`> (where `E` implements [`std::error::Error`]) into a [`Result`]<_, [`Box`]<[`dyn`][`std::error::Error`]` + 'static`>>
pub trait BoxResult2Result<T, E> {
    fn boxed(self) -> Result<T, Box<dyn std::error::Error>>;
}
/// Implements the [`BoxResult2Result`] trait for all [`std::error::Error`] types.
impl <T, E: std::error::Error + 'static> BoxResult2Result<T, E> for Result<T, E> {
    /// Converts into a [`Box`] and wraps it in a [`Result`].
    /// 
    /// # Arguments
    /// 
    /// * [`self`] - The datatype that implements [`std::error::Error`] to be wrapped in a [`Box`].
    /// 
    /// # Returns
    /// 
    /// The converted [`Result`]<(), [`Box`]<[`dyn`][`std::error::Error`]` + 'static`>>.
    /// 
    /// # Example
    /// 
    /// TODO fix this example
    /// 
    /// ```ignore
    /// use crosstalk::BoxResult2Result;
    /// use std::error::Error;
    /// 
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let result = Error::from("some error".to_string());
    ///     let boxed_error = result.boxed();
    ///     assert_eq!(result, Err(boxed_error.unwrap_err()));
    ///     assert_eq!(boxed_error.unwrap_err().to_string(), "some error");
    ///     boxed_error
    /// }
    /// ```
    fn boxed(self) -> Result<T, Box<dyn std::error::Error>> {
        self.map_err(|e| e.into())
    }
}
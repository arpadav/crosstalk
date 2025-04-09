#![doc(html_root_url = "https://docs.rs/crosstalk/1.0")]
#![doc = include_str!("../README.md")]
// --------------------------------------------------
// external
// --------------------------------------------------
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use tokio::sync::broadcast::{
    Sender as TokioSender,
    Receiver as TokioReceiver,
};

// --------------------------------------------------
// local
// --------------------------------------------------
pub use crosstalk_macros::init;
pub use crosstalk_macros::AsTopic;

// --------------------------------------------------
// re-exports
// --------------------------------------------------
pub mod __macro_exports {
    pub use tokio::runtime;
    pub use tokio::sync::broadcast;

    #[inline(always)]
    /// Downcasts a [`Box`] into a type `T`
    /// 
    /// # Arguments
    /// 
    /// * `buf` - the [`Box`] to downcast
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crosstalk::__macro_exports::downcast;
    /// 
    /// let mut buf = Box::new(5) as Box<dyn std::any::Any + 'static>;
    /// assert_eq!(downcast::<i32>(buf, crosstalk::Error::PublisherMismatch("foo", "bar")).unwrap(), 5);
    /// ```
    pub fn downcast<T>(buf: Box<dyn std::any::Any + 'static>, on_error: crate::Error) -> Result<T, crate::Error>
    where
        T: 'static,
    {
        match buf.downcast::<T>() {
            Ok(t) => Ok(*t),
            Err(_) => Err(on_error),
        }
    }
}

/// A trait bound an enum as a [`CrosstalkTopic`]
pub trait CrosstalkTopic: Eq + Copy + Clone + PartialEq + std::hash::Hash {}

/// A trait to bound a datatype as a [`CrosstalkData`]
pub trait CrosstalkData: Clone + Send + 'static {}
/// [`CrosstalkData`] implementation for all types
impl<T: Clone + Send + 'static> CrosstalkData for T {}

#[derive(Copy, Clone, Debug)]
/// [`crosstalk`](crate) errors
pub enum Error {
    PublisherMismatch(&'static str, &'static str),
    SubscriberMismatch(&'static str, &'static str),
}
/// [`crosstalk::Error`](crate::Error) implementation of [`std::error::Error`]
impl std::error::Error for Error {}
/// [`crosstalk::Error`](crate::Error) implementation of [`std::fmt::Display`]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PublisherMismatch(input, output) => write!(f, "Publisher type mismatch: {} (cast) != {} (expected)", input, output),
            Error::SubscriberMismatch(input, output) => write!(f, "Subscriber type mismatch: {} (cast) != {} (expected)", input, output),
        }
    }
}

/// A trait to define a [`CrosstalkPubSub`]
/// 
/// This is used to implement the [`CrosstalkPubSub`] trait
/// using the [`crosstalk_macros::init!`] macro
/// for the [`ImplementedBoundedNode`] struct
/// 
/// This is not meant to be used directly, and is automatically
/// implemented when calling [`crosstalk_macros::init!`]
pub trait CrosstalkPubSub<T> {
    fn publisher<D: CrosstalkData>(&mut self, topic: T) -> Result<Publisher<D, T>, crate::Error>;
    
    fn subscriber<D: CrosstalkData>(&mut self, topic: T) -> Result<Subscriber<D, T>, crate::Error>;
    
    #[allow(clippy::type_complexity)]
    fn pubsub<D: CrosstalkData>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), crate::Error>;
}

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
/// let mut node = crosstalk::BoundedNode::<House>::new(10);
/// let (pub0, mut sub0) = node.pubsub_blocking(House::Bedroom).unwrap();
/// let (pub1, mut sub1) = node.pubsub_blocking(House::Bedroom).unwrap();
/// 
/// pub0.write("Hello".to_string());
/// pub0.write("World".to_string());
/// pub1.write("Foo".to_string());
/// pub1.write("Bar".to_string());
/// 
/// assert_eq!(sub1.try_read().unwrap(), "Hello");
/// assert_eq!(sub1.try_read().unwrap(), "World");
/// assert_eq!(sub1.try_read().unwrap(), "Foo");
/// assert_eq!(sub1.try_read().unwrap(), "Bar");
/// 
/// assert_eq!(sub0.try_read().unwrap(), "Hello");
/// assert_eq!(sub0.try_read().unwrap(), "World");
/// assert_eq!(sub0.try_read().unwrap(), "Foo");
/// assert_eq!(sub0.try_read().unwrap(), "Bar");
/// ```
pub struct BoundedNode<T> {
    pub node: Arc<Mutex<ImplementedBoundedNode<T>>>,
    pub size: usize,
}
/// [`BoundedNode`] implementation 
/// 
/// This holds an [`Arc<Mutex<ImplementedBoundedNode<T>>>`], which
/// references the true (private) node that implements the [`AsTopic`] trait.
impl<T> BoundedNode<T> 
where
    T: CrosstalkTopic,
    ImplementedBoundedNode<T>: CrosstalkPubSub<T>,
{
    #[inline(always)]
    /// Creates a new [`BoundedNode`]
    /// 
    /// # Arguments
    /// 
    /// * `size` - the size of each buffer
    /// 
    /// # Panics
    /// 
    /// Panics if `size` is 0. This is intentional because of two reasons:
    /// 
    /// 1. No buffer can have a size of 0.
    /// 2. A [`tokio::sync::broadcast::Sender`] cannot be created with a size of 0,
    ///    and therefore, a [`BoundedNode`] could potentially be created without
    ///    error but then calling [`BoundedNode::publisher`] or [`BoundedNode::subscriber`]
    ///    would result in a panic later on.
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
        if size == 0 {
            panic!("Size must be greater than 0. Attempting to make `tokio::sync::broadcast::channels` later will result in a panic.");
        }
        Self {
            node: Arc::new(Mutex::new(ImplementedBoundedNode::<T>::new(size))),
            size,
        }
    }

    #[inline(always)]
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
    ///     assert!(node.publisher::<String>(House::Bedroom).await.is_ok());
    /// }
    /// ```
    pub async fn publisher<D: CrosstalkData>(&mut self, topic: T) -> Result<Publisher<D, T>, crate::Error> {
        self.node.lock().await.publisher(topic)
    }

    #[inline(always)]
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// assert!(node.publisher_blocking::<String>(House::Bedroom).is_ok());
    /// ```
    pub fn publisher_blocking<D: CrosstalkData>(&mut self, topic: T) -> Result<Publisher<D, T>, crate::Error> {
        self.node.blocking_lock().publisher(topic)
    }

    #[inline(always)]
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
    ///     assert!(node.subscriber::<String>(House::Bedroom).await.is_ok());
    /// }
    /// ```
    pub async fn subscriber<D: CrosstalkData>(&mut self, topic: T) -> Result<Subscriber<D, T>, crate::Error> {
        self.node.lock().await.subscriber(topic)
    }

    #[inline(always)]
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// assert!(node.subscriber_blocking::<String>(House::Bedroom).is_ok());
    /// ```
    pub fn subscriber_blocking<D: CrosstalkData>(&mut self, topic: T) -> Result<Subscriber<D, T>, crate::Error> {
        self.node.blocking_lock().subscriber(topic)
    }

    #[inline(always)]
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
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut node = crosstalk::BoundedNode::<House>::new(10);
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).await.unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.try_read().unwrap(), "hello");
    /// }
    /// ```
    /// 
    pub async fn pubsub<D: CrosstalkData>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), crate::Error> {
        self.node.lock().await.pubsub(topic)
    }

    #[inline(always)]
    #[allow(clippy::type_complexity)]
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// let (publisher, mut subscriber) = node.pubsub_blocking(House::Bedroom).unwrap();
    /// publisher.write("hello".to_string());
    /// assert_eq!(subscriber.try_read().unwrap(), "hello");
    /// ```
    /// 
    pub fn pubsub_blocking<D: CrosstalkData>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), crate::Error> {
        self.node.blocking_lock().pubsub(topic)
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
/// * `size` - the size of each buffer
pub struct ImplementedBoundedNode<T> {
    pub senders: HashMap<T, Box<dyn std::any::Any + 'static>>,
    pub size: usize,
}

/// [`ImplementedBoundedNode`] implementation of [`Send`]
unsafe impl<T> Send for ImplementedBoundedNode<T> {}
/// [`ImplementedBoundedNode`] implementation of [`Sync`]
unsafe impl<T> Sync for ImplementedBoundedNode<T> {}

/// [`ImplementedBoundedNode`] implementation 
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
            size,
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
    buf: TokioSender<D>,
}
/// [`Publisher`] implementation
impl<D, T> Publisher<D, T> {
    #[inline(always)]
    /// See [`BoundedNode::publisher`]
    pub fn new(topic: T, buf: TokioSender<D>) -> Self {
        Self { topic, buf }
    }

    #[inline(always)]
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// let (publisher, mut subscriber) = node.pubsub_blocking(House::Bedroom).unwrap();
    /// publisher.write("hello".to_string());
    /// std::thread::spawn(move || {
    ///     assert_eq!(subscriber.try_read().unwrap(), "hello");
    /// });
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
    sndr: Arc<TokioSender<D>>,
}
/// [`Subscriber`] implementation 
impl<D: Clone, T: Clone> Subscriber<D, T> {
    #[inline(always)]
    /// See [`BoundedNode::subscriber`]
    pub fn new(
        topic: T,
        rcvr: Option<TokioReceiver<D>>,
        sndr: Arc<TokioSender<D>>,
    ) -> Self {
        Self {
            topic,
            rcvr: Receiver::new(rcvr.unwrap_or(sndr.subscribe())),
            sndr: sndr.clone(),
        }
    }

    #[inline(always)]
    /// Asynchronous blocking read from the [`TokioReceiver`]
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
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).await.unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.read().await, Some("hello".to_string()));
    /// }
    /// ```
    pub async fn read(&mut self) -> Option<D> {
        self.rcvr.read().await
    }
    
    #[inline(always)]
    /// Non-blocking read from [`TokioReceiver`]
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// let (publisher, mut subscriber) = node.pubsub_blocking(House::Bedroom).unwrap();
    /// publisher.write("hello".to_string());
    /// assert_eq!(subscriber.try_read(), Some(String::from("hello")));
    /// assert_eq!(subscriber.try_read(), None);
    /// ```
    pub fn try_read(&mut self) -> Option<D> {
        self.rcvr.try_read()
    }

    #[inline(always)]
    /// Non-blocking read from [`TokioReceiver`], returning
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// let (publisher, mut subscriber) = node.pubsub_blocking(House::Bedroom).unwrap();
    /// publisher.write("hello".to_string());
    /// assert_eq!(subscriber.try_read_raw(), Some(String::from("hello")));
    /// assert_eq!(subscriber.try_read_raw(), None);
    /// ```
    pub fn try_read_raw(&mut self) -> Option<D> {
        self.rcvr.try_read_raw()
    }
    
    #[inline(always)]
    /// Sequential blocking read from [`TokioReceiver`]
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// let (publisher, mut subscriber) = node.pubsub_blocking(House::Bedroom).unwrap();
    /// publisher.write("hello".to_string());
    /// assert_eq!(subscriber.read_blocking(), Some("hello".to_string()));
    /// ```
    pub fn read_blocking(&mut self) -> Option<D> {
        self.rcvr.read_blocking()
    }
    
    #[inline(always)]
    /// Asynchronous non-blocking read from [`TokioReceiver`]
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
    ///     let (publisher, mut subscriber) = node.pubsub(House::Bedroom).await.unwrap();
    ///     publisher.write("hello".to_string());
    ///     assert_eq!(subscriber.read_timeout(std::time::Duration::from_millis(100)).await, Some("hello".to_string()));
    /// }
    /// ```
    pub async fn read_timeout(&mut self, timeout: std::time::Duration) -> Option<D> {
        self.rcvr.read_timeout(timeout).await
    }
}
/// [`Subscriber`] implementation of [`Clone`]
impl<D: Clone, T: Clone> Clone for Subscriber<D, T> {
    #[inline(always)]
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
    /// let mut node = crosstalk::BoundedNode::<House>::new(10);
    /// let (publisher, mut subscriber) = node.pubsub_blocking(House::Bedroom).unwrap();
    /// let mut subscriber_2 = subscriber.clone();
    /// publisher.write("hello".to_string());
    /// assert_eq!(subscriber.try_read().unwrap(), "hello");
    /// assert_eq!(subscriber_2.try_read().unwrap(), "hello");
    /// ```
    fn clone(&self) -> Self {
        Self {
            topic: self.topic.clone(),
            rcvr: Receiver::new(self.sndr.subscribe()),
            sndr: self.sndr.clone(),
        }
    }
}

/// Receiver
/// 
/// Define a receiver for subscribing messages
/// 
/// Reads from [`TokioReceiver`]
struct Receiver<D> {
    buf: TokioReceiver<D>,
}
/// [`Receiver`] implementation
impl<D: Clone> Receiver<D>{
    #[inline(always)]
    /// Constructs a new [`Receiver`]
    pub fn new(
        buf: TokioReceiver<D>,
    ) -> Self {
        Self { buf }
    }

    /// Reads from [`TokioReceiver`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::read`]
    pub async fn read(&mut self) -> Option<D> {
        loop {
            match self.buf.recv().await {
                Ok(res) => return Some(res),
                Err(e) => match e {
                    tokio::sync::broadcast::error::RecvError::Lagged(_) => { continue; }

                    #[cfg(not(any(feature = "log", feature = "tracing")))]
                    _ => return None,

                    #[cfg(any(feature = "log", feature = "tracing"))]
                    _ => {
                        #[cfg(feature = "log")]
                        log::error!("{}", e);
                        #[cfg(feature = "tracing")]
                        tracing::error!("{}", e);
                        return None
                    }
                }
            }
        }
    }    

    /// Reads from [`TokioReceiver`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::try_read`]
    pub fn try_read(&mut self) -> Option<D> {
        loop {
            match self.buf.try_recv() {
                Ok(d) => return Some(d),
                Err(e) => {
                    match e {
                        tokio::sync::broadcast::error::TryRecvError::Lagged(_) => { continue; },

                        #[cfg(not(any(feature = "log", feature = "tracing")))]
                        _ => return None,

                        #[cfg(any(feature = "log", feature = "tracing"))]
                        _ => {
                            #[cfg(feature = "log")]
                            log::error!("{}", e);
                            #[cfg(feature = "tracing")]
                            tracing::error!("{}", e);
                            return None
                        },
                    }
                },
            }
        } 
    }

    /// Reads from [`TokioReceiver`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::try_read_raw`]
    pub fn try_read_raw(&mut self) -> Option<D> {
        match self.buf.try_recv() {
            Ok(d) => Some(d),

            #[cfg(not(any(feature = "log", feature = "tracing")))]
            Err(_) => None,
            
            #[cfg(any(feature = "log", feature = "tracing"))]
            Err(e) => {
                #[cfg(feature = "log")]
                log::error!("{}", e);
                #[cfg(feature = "tracing")]
                tracing::error!("{}", e);
                None
            },
        }
    }
    
    /// Reads from [`TokioReceiver`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::read_blocking`]
    pub fn read_blocking(&mut self) -> Option<D> {
        loop {
            match self.buf.blocking_recv() {
                Ok(res) => return Some(res),
                Err(e) => match e {
                    tokio::sync::broadcast::error::RecvError::Lagged(_) => { continue; }
                    
                    #[cfg(not(any(feature = "log", feature = "tracing")))]
                    _ => return None,

                    #[cfg(any(feature = "log", feature = "tracing"))]
                    _ => {
                        #[cfg(feature = "log")]
                        log::error!("{}", e);
                        #[cfg(feature = "tracing")]
                        tracing::error!("{}", e);
                        return None
                    },
                }
            }
        }
    }
    
    /// Reads from [`TokioReceiver`]
    /// 
    /// This struct/function is not meant to be used directly,
    /// rather through the [`Subscriber`] struct with [`Subscriber::read_timeout`]
    pub async fn read_timeout(&mut self, timeout: std::time::Duration) -> Option<D> {
        match tokio::runtime::Handle::try_current() {
            Ok(_) => {
                match tokio::time::timeout(timeout, self.buf.recv()).await {
                    Ok(res) => {
                        match res {
                            Ok(res) => Some(res),

                            #[cfg(not(any(feature = "log", feature = "tracing")))]
                            Err(_) => None,
                            
                            #[cfg(any(feature = "log", feature = "tracing"))]
                            Err(e) => {
                                #[cfg(feature = "log")]
                                log::error!("{}", e);
                                #[cfg(feature = "tracing")]
                                tracing::error!("{}", e);
                                None
                            },
                        }
                    },

                    #[cfg(not(any(feature = "log", feature = "tracing")))]
                    Err(_) => None,
                    
                    #[cfg(any(feature = "log", feature = "tracing"))]
                    Err(e) => {
                        #[cfg(feature = "log")]
                        log::error!("{}", e);
                        #[cfg(feature = "tracing")]
                        tracing::error!("{}", e);
                        None
                    },
                }
            },

            #[cfg(not(any(feature = "log", feature = "tracing")))]
            Err(_) => None,
            
            #[cfg(any(feature = "log", feature = "tracing"))]
            Err(e) => {
                #[cfg(feature = "log")]
                log::error!("{}", e);
                #[cfg(feature = "tracing")]
                tracing::error!("{}", e);
                None
            },
        }
    }
}

// --------------------------------------------------
// for testing
// --------------------------------------------------
#[allow(unused_imports)]
use crosstalk_macros::init_test;
#[allow(unused_imports)]
use crosstalk_macros::AsTopicTest;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(AsTopicTest)]
    enum TestTopic {
        A,
        B,
        C,
    }
    super::init_test! {
        TestTopic::A => String,
        TestTopic::B => bool,
        TestTopic::C => i32,
    }

    #[derive(AsTopicTest)]
    enum AnotherTestTopic {
        Foo,
        Bar,
    }
    super::init_test! {
        AnotherTestTopic::Foo => Vec<String>,
        AnotherTestTopic::Bar => Vec<bool>,
    }

    #[test]
    fn test_single_pubsub_blocking() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub_blocking(TestTopic::A).unwrap();
        publisher.write("test".to_string());
        assert_eq!(subscriber.try_read().unwrap(), "test");
    }

    #[test]
    fn test_multiple_subscribers_blocking() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut sub1) = node.pubsub_blocking(TestTopic::A).unwrap();
        let mut sub2 = node.subscriber_blocking::<String>(TestTopic::A).unwrap();

        publisher.write("hello".to_string());
        assert_eq!(sub1.try_read().unwrap(), "hello");
        assert_eq!(sub2.try_read().unwrap(), "hello");
    }

    #[test]
    fn test_cross_topic_isolation() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (pub_a, mut sub_a) = node.pubsub_blocking(TestTopic::A).unwrap();
        let (pub_b, mut sub_b) = node.pubsub_blocking(TestTopic::B).unwrap();

        pub_a.write("string".to_string());
        pub_b.write(true);

        assert_eq!(sub_a.try_read().unwrap(), "string");
        assert!(sub_b.try_read().unwrap());
        assert!(sub_a.try_read().is_none());
        assert!(sub_b.try_read().is_none());
    }

    #[test]
    fn test_multiple_threads_blocking() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub_blocking(TestTopic::A).unwrap();

        let handle = std::thread::spawn(move || {
            publisher.write("threaded".to_string());
        });

        handle.join().unwrap();
        assert_eq!(subscriber.try_read().unwrap(), "threaded");
    }

    #[tokio::test]
    async fn test_async_pubsub_single_runtime() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub(TestTopic::A).await.unwrap();
        publisher.write("async".to_string());
        assert_eq!(subscriber.read().await.unwrap(), "async");
    }

    #[test]
    fn test_high_volume_blocking() {
        let mut node = BoundedNode::<TestTopic>::new(100);
        let (publisher, mut subscriber) = node.pubsub_blocking(TestTopic::C).unwrap();

        for i in 0..100 {
            publisher.write(i);
        }

        for i in 0..100 {
            assert_eq!(subscriber.try_read().unwrap(), i);
        }
        assert!(subscriber.try_read().is_none());
    }

    #[test]
    fn test_cloned_subscribers() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut sub1) = node.pubsub_blocking(TestTopic::A).unwrap();
        let mut sub2 = sub1.clone();

        publisher.write("clone".to_string());
        assert_eq!(sub1.try_read().unwrap(), "clone");
        assert_eq!(sub2.try_read().unwrap(), "clone");
    }

    #[test]
    fn test_buffer_overflow_handling() {
        let mut node = BoundedNode::<TestTopic>::new(2);
        let (publisher, mut subscriber) = node.pubsub_blocking(TestTopic::A).unwrap();

        publisher.write("msg1".to_string());
        publisher.write("msg2".to_string());
        publisher.write("msg3".to_string());

        assert_eq!(subscriber.try_read().unwrap(), "msg2");
        assert_eq!(subscriber.try_read().unwrap(), "msg3");
        assert!(subscriber.try_read().is_none());
    }

    #[test]
    fn test_type_mismatch_errors() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        
        let publisher_res = node.publisher_blocking::<i32>(TestTopic::A);
        assert!(matches!(publisher_res, Err(Error::PublisherMismatch(_, _))));
        
        let subscriber_res = node.subscriber_blocking::<i32>(TestTopic::A);
        assert!(matches!(subscriber_res, Err(Error::SubscriberMismatch(_, _))));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_multiple_async_runtimes() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub(TestTopic::A).await.unwrap();

        let handle = tokio::spawn(async move {
            publisher.write("async".to_string());
        });

        handle.await.unwrap();
        assert_eq!(subscriber.read().await.unwrap(), "async");
    }

    #[test]
    fn test_mixed_async_blocking() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub_blocking(TestTopic::A).unwrap();

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            publisher.write("mixed".to_string());
        });

        assert_eq!(subscriber.try_read().unwrap(), "mixed");
    }

    #[test]
    fn test_complex_data_types() {
        let mut node = BoundedNode::<AnotherTestTopic>::new(10);
        let (pub_foo, mut sub_foo) = node.pubsub_blocking(AnotherTestTopic::Foo).unwrap();
        let (pub_bar, mut sub_bar) = node.pubsub_blocking(AnotherTestTopic::Bar).unwrap();

        pub_foo.write(vec!["a".to_string(), "b".to_string()]);
        pub_bar.write(vec![true, false]);

        assert_eq!(sub_foo.try_read().unwrap(), vec!["a", "b"]);
        assert_eq!(sub_bar.try_read().unwrap(), vec![true, false]);
    }

    #[test]
    fn test_concurrent_publishers() {
        let mut node = BoundedNode::<TestTopic>::new(100);
        let (pub1, mut sub) = node.pubsub_blocking(TestTopic::C).unwrap();
        let pub2 = node.publisher_blocking::<i32>(TestTopic::C).unwrap();

        let handle1 = std::thread::spawn(move || {
            for i in 0..50 {
                pub1.write(i);
            }
        });

        let handle2 = std::thread::spawn(move || {
            for i in 50..100 {
                pub2.write(i);
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let mut received = Vec::new();
        while let Some(msg) = sub.try_read() {
            received.push(msg);
        }
        assert_eq!(received.len(), 100);
    }

    #[tokio::test]
    async fn test_async_subscriber_cloning() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut sub1) = node.pubsub(TestTopic::A).await.unwrap();
        let mut sub2 = sub1.clone();

        publisher.write("async_clone".to_string());
        assert_eq!(sub1.read().await.unwrap(), "async_clone");
        assert_eq!(sub2.read().await.unwrap(), "async_clone");
    }

    #[test]
    fn test_dropped_publisher_behavior() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub_blocking::<String>(TestTopic::A).unwrap();
        drop(publisher);
        assert!(subscriber.try_read().is_none());
    }

    #[tokio::test]
    async fn test_multiple_async_publishers() {
        const LOOP_COUNT: usize = 10;
        const BUFFER_SIZE: usize = 10;
        let mut node = BoundedNode::<TestTopic>::new(BUFFER_SIZE);
        let (publisher, mut subscriber) = node.pubsub(TestTopic::A).await.unwrap();
        let publisher_1 = publisher.clone();
        let publisher_2 = publisher.clone();

        let task1 = tokio::spawn({
            async move {
                for _ in 0..LOOP_COUNT {
                    publisher_1.write("task1".to_string());
                }
            }
        });

        let task2 = tokio::spawn({
            async move {
                for _ in 0..LOOP_COUNT {
                    publisher_2.write("task2".to_string());
                }
            }
        });

        let _ = tokio::join!(task1, task2);

        let mut task1_count = 0;
        let mut task2_count = 0;
        for _ in 0..BUFFER_SIZE {
            let msg = subscriber.read().await.unwrap();
            if msg == "task1" { task1_count += 1; }
            if msg == "task2" { task2_count += 1; }
        }
        assert_eq!(task1_count + task2_count, BUFFER_SIZE);
    }

    #[test]
    fn test_blocking_read_with_delay() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub_blocking(TestTopic::A).unwrap();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            publisher.write("delayed".to_string());
        });

        assert_eq!(subscriber.read_blocking().unwrap(), "delayed");
    }

    #[tokio::test]
    async fn test_read_timeout_behavior() {
        let mut node = BoundedNode::<TestTopic>::new(10);
        let (publisher, mut subscriber) = node.pubsub(TestTopic::A).await.unwrap();

        let timeout = std::time::Duration::from_millis(100);
        assert!(subscriber.read_timeout(timeout).await.is_none());

        publisher.write("timeout_test".to_string());
        assert_eq!(subscriber.read_timeout(timeout).await.unwrap(), "timeout_test");
    }

    #[test]
    fn test_multiple_topics_concurrently() {
        let mut node = BoundedNode::<AnotherTestTopic>::new(10);
        let (pub_foo, mut sub_foo) = node.pubsub_blocking(AnotherTestTopic::Foo).unwrap();
        let (pub_bar, mut sub_bar) = node.pubsub_blocking(AnotherTestTopic::Bar).unwrap();

        let handle1 = std::thread::spawn(move || {
            pub_foo.write(vec!["thread".to_string()]);
        });

        let handle2 = std::thread::spawn(move || {
            pub_bar.write(vec![true]);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        assert_eq!(sub_foo.try_read().unwrap(), vec!["thread"]);
        assert_eq!(sub_bar.try_read().unwrap(), vec![true]);
    }

    #[test]
    #[should_panic]
    fn test_zero_capacity_node() {
        let _ = BoundedNode::<TestTopic>::new(0);
    }

    #[tokio::test]
    async fn test_async_unbounded_messaging() {
        let mut node = BoundedNode::<TestTopic>::new(1000);
        let (publisher, mut subscriber) = node.pubsub(TestTopic::A).await.unwrap();

        let messages = vec!["msg1", "msg2", "msg3", "msg4", "msg5"];
        for msg in &messages {
            publisher.write(msg.to_string());
        }

        for expected in messages {
            assert_eq!(subscriber.read().await.unwrap(), expected);
        }
    }

    #[test]
    fn test_error_handling_lagged_messages() {
        let mut node = BoundedNode::<TestTopic>::new(2);
        let (publisher, mut subscriber) = node.pubsub_blocking(TestTopic::A).unwrap();

        for i in 0..5 {
            publisher.write(format!("msg{}", i));
        }

        let mut received = Vec::new();
        while let Some(msg) = subscriber.try_read() {
            received.push(msg);
        }
        assert_eq!(received, vec!["msg3", "msg4"]);
    }
}

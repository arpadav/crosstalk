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
pub struct BoundedNode<T> {
    pub node: Arc<Mutex<ImplementedBoundedNode<T>>>,
    pub size: usize,
}

impl<T> BoundedNode<T> 
where
    T: CrosstalkTopic,
    ImplementedBoundedNode<T>: CrosstalkPubSub<T>,
{
    #[inline]
    pub fn new(size: usize) -> Self {
        Self {
            node: Arc::new(Mutex::new(ImplementedBoundedNode::<T>::new(size.clone()))),
            size,
        }
    }

    #[inline]
    pub fn publisher<D: 'static>(&mut self, topic: T) -> Result<Publisher<D, T>, Box<dyn std::error::Error>> {
        let mut n = self.node.lock().unwrap();
        n.publisher(topic)
    }

    #[inline]
    pub fn subscriber<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<Subscriber<D, T>, Box<dyn std::error::Error>> {
        let mut n = self.node.lock().unwrap();
        n.subscriber(topic)
    }

    #[inline]
    pub fn pubsub<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), Box<dyn std::error::Error>> {
        let mut n = self.node.lock().unwrap();
        n.pubsub(topic)
    }

    #[inline]
    pub fn delete_publisher<D: 'static>(&mut self, _publisher: Publisher<D, T>) {
        let mut n = self.node.lock().unwrap();
        n.delete_publisher(_publisher)
    }

    #[inline]
    pub fn delete_subscriber<D: Clone + Send + 'static>(&mut self, subscriber: Subscriber<D, T>) {
        let mut n = self.node.lock().unwrap();
        n.delete_subscriber(subscriber)
    }
}


pub struct ImplementedBoundedNode<T> {
    pub senders: HashMap<T, Box<dyn Any + 'static>>,
    pub runtimes: HashMap<T, Arc<tokio::runtime::Runtime>>,
    pub create_runtimes: bool,
    pub size: usize,
}

// TODO: make this safe?
unsafe impl<T> Send for ImplementedBoundedNode<T> {}
unsafe impl<T> Sync for ImplementedBoundedNode<T> {}

impl<T> ImplementedBoundedNode<T>
where
    T: CrosstalkTopic,
{
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
pub struct Publisher<D, T> {
    pub topic: T,
    buf: tokio::sync::broadcast::Sender<D>,
}
impl<D, T> Publisher<D, T> {
    #[inline]
    pub fn new(buf: tokio::sync::broadcast::Sender<D>, topic: T) -> Self {
        Self { buf, topic }
    }

    #[inline]
    pub fn write(&self, sample: D) {
        let _ = self.buf.send(sample);
        // let res = self.buf.send(sample);
        // match res {
        //     Ok(r) => println!("write: {:?}", r),
        //     Err(e) => println!("write error: {}", e),
        // }
    }
}


pub struct Subscriber<D, T> {
    pub topic: T,
    rcvr: Receiver<D>,
    sndr: Arc<tokio::sync::broadcast::Sender<D>>,
    rt: Option<Arc<tokio::runtime::Runtime>>,
}

impl<D, T> Subscriber<D, T> 
where
    T: Clone,
    D: Clone,
{
    #[inline]
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

    pub async fn read(&mut self) -> Option<D> {
        self.rcvr.read().await
    }
    
    #[inline]
    pub fn try_read(&mut self) -> Option<D> {
        self.rcvr.try_read()
    }

    #[inline]
    pub fn try_read_raw(&mut self) -> Option<D> {
        self.rcvr.try_read_raw()
    }
    
    #[inline]
    pub fn read_blocking(&mut self) -> Option<D> {
        self.rcvr.read_blocking()
    }
    
    #[inline]
    pub async fn read_timeout(&mut self, timeout: std::time::Duration) -> Option<D> {
        self.rcvr.read_timeout(timeout).await
    }

    #[inline]
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
    pub fn try_read_raw(&mut self) -> Option<D> {
        let _guard = match self.rt {
            Some(ref rt) => Some(rt.enter()),
            None => None,
        };
        let res = self.buf.try_recv();
        match res {
            Ok(d) => Some(d),
            Err(_) => None,
        }
    }
    
    #[inline]
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
    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }
}

#[derive(Debug)]
/// Error
/// 
/// Crosstalk errors
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

pub trait CrosstalkTopic: Eq + Hash + Copy + Clone + PartialEq {}

pub trait CrosstalkPubSub<T> {
    fn publisher<D: 'static>(&mut self, topic: T) -> Result<Publisher<D, T>, Box<dyn std::error::Error>>;
    fn subscriber<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<Subscriber<D, T>, Box<dyn std::error::Error>>;
    fn pubsub<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), Box<dyn std::error::Error>>;
    fn delete_publisher<D: 'static>(&mut self, _publisher: Publisher<D, T>);
    fn delete_subscriber<D: Clone + Send + 'static>(&mut self, subscriber: Subscriber<D, T>);
}


#[inline]
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
/// Implements the [`BoxResult2Result`](crate::sflib::errors::BoxResult2Result) trait for all [`std::error::Error`](std::error::Error) types.
impl <T, E: std::error::Error + 'static> BoxResult2Result<T, E> for Result<T, E> {
    /// Converts into a [`Box`](std::boxed::Box) and wraps it in a [`Result`](std::result::Result).
    /// 
    /// # Arguments
    /// 
    /// * [`self`] - The datatype that implements [`std::error::Error`] to be wrapped in a [`Box`](std::boxed::Box).
    /// 
    /// # Returns
    /// 
    /// The converted [`Result`]<(), [`Box`]<[`dyn`][`std::error::Error`]` + 'static`>>.
    /// 
    /// # Usage
    /// 
    /// TODO
    fn boxed(self) -> Result<T, Box<dyn std::error::Error>> {
        self.map_err(|e| e.into())
    }
}
// --------------------------------------------------
// external
// --------------------------------------------------
use std::{
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering,
            AtomicUsize,
        },
    },
    any::Any,
    collections::HashMap,
};
use core::hash::Hash;

// --------------------------------------------------
// internal
// --------------------------------------------------
// pub use crosstalk_macros::bounded;
pub use crosstalk_macros::unbounded;


pub struct UnboundedCommonNode<T> {
    pub senders: HashMap<T, Box<dyn Any + 'static>>,
    pub receivers: HashMap<T, Box<dyn Any + 'static>>,
    pub distributors: HashMap<T, HashMap<usize, Box<dyn Any>>>,
    pub num_dist_per_topic: HashMap<T, Arc<AtomicUsize>>,
    pub uniq_dist_id_incr: HashMap<T, usize>,
    pub termination_chnls: HashMap<T, (flume::Sender<usize>, flume::Receiver<usize>)>,
    pub forwarding_flags: HashMap<T, Arc<AtomicBool>>,
}
impl<T> UnboundedCommonNode<T>
where
    T: Eq + Hash + Copy + Clone + PartialEq,
{

    pub fn new() -> Self {
        Self {
            senders: HashMap::new(),
            receivers: HashMap::new(),
            distributors: HashMap::new(),
            num_dist_per_topic: HashMap::new(),
            uniq_dist_id_incr: HashMap::new(),
            termination_chnls: HashMap::new(),
            forwarding_flags: HashMap::new(),
        }
    }


    pub fn restart_forwarding(&mut self, topic: &T, ndist: Option<usize>) -> (Arc<AtomicBool>, flume::Receiver<usize>) {
        // --------------------------------------------------
        // if the termination channel exists:
        // - get the number of distributors for the topic
        // - get the termination channel
        // - send the number of distributors for termination
        // - get the forwarding boolean to confirm termination\
        // - return
        // --------------------------------------------------
        // otherwise:
        // - create the termination channel
        // - create the forwarding boolean
        // - return
        // --------------------------------------------------
        if self.termination_chnls.contains_key(&topic) {
            let ndist = match ndist {
                Some(ndist) => ndist,
                None => self.num_dist_per_topic.get(&topic).unwrap().load(Ordering::SeqCst) - 1,
            };
            let (sender, receiver) = self.termination_chnls.get_mut(&topic).unwrap();
            let res = sender.send(ndist);
            tracing::debug!("Channel termination requested from main thread: {:?}", res);
            let fflag = self.forwarding_flags.get(&topic).unwrap().clone();
            (fflag, receiver.clone())
        } else {
            let (sender, receiver) = flume::unbounded();
            let fflag = Arc::new(AtomicBool::new(false));
            self.termination_chnls.insert(topic.clone(), (sender, receiver.clone()));
            tracing::debug!("New sender/receiver pair created");
            self.forwarding_flags.insert(topic.clone(), fflag.clone());
            (fflag, receiver)
        }
    }


    fn get_flume_receiver<D: 'static>(&mut self, topic: &T) -> flume::Receiver<D> {
        let frecv_ = downcast::<flume::Receiver<D>>(self.receivers.remove(&topic).unwrap()).unwrap();
        let frec = frecv_.clone();
        self.receivers.insert(topic.clone(), Box::new(frecv_));
        frec
    }


    fn get_vectorized_distributors<D: 'static>(&mut self, topic: &T) -> Vec::<flume::Sender<D>> {
        let mut dists: Vec::<flume::Sender<D>> = Vec::new();
        // --------------------------------------------------
        // remove the distributors for the topic
        // --------------------------------------------------
        let mut distributors_copy: HashMap<usize, Box<dyn std::any::Any>> = HashMap::new();
        let distributors = self.distributors.remove(&topic).unwrap();
        for (cdid, cd) in distributors {
            let dcd = downcast::<flume::Sender<D>>(cd).unwrap();
            dists.push(dcd.clone());
            // --------------------------------------------------
            // add to copy
            // --------------------------------------------------
            distributors_copy.insert(cdid, Box::new(dcd));
        }
        // --------------------------------------------------
        // insert back and return
        // --------------------------------------------------
        self.distributors.insert(topic.clone(), distributors_copy);
        dists
    }


    pub fn update_distribution_threads<D: Send + Clone + 'static>(&mut self, topic: &T, ndist: Option<usize>) {
        let (fflag, tchnl) = self.restart_forwarding(&topic, ndist);
        let frec = self.get_flume_receiver::<D>(&topic);
        let dists = self.get_vectorized_distributors(&topic);
        // --------------------------------------------------
        // create the buffer forwarding
        // - move everything into new thread once the termination
        //   has been set to confirm the previous thread has 
        //   been terminated
        // --------------------------------------------------
        tracing::debug!("Termination command sent, waiting for thread to stop...");
        while fflag.load(std::sync::atomic::Ordering::SeqCst) { std::thread::sleep(std::time::Duration::from_nanos(10)); }
        let fflag_ = fflag.clone();
        tracing::debug!("Thread STOPPED!");
        tracing::debug!("Spawning thread with {} distributor(s)...", dists.len());
        std::thread::spawn(move || forward::<D>(frec, dists, fflag_, tchnl));
        while !fflag.load(std::sync::atomic::Ordering::SeqCst) { std::thread::sleep(std::time::Duration::from_nanos(10)); }
        tracing::debug!("Thread has started!");
    }
}


#[derive(Clone)]
pub struct Publisher<D, T> {
    buf: flume::Sender<D>,
    pub topic: T
}
impl<D, T> Publisher<D, T> {
    pub fn new(buf: flume::Sender<D>, topic: T) -> Self {
        Self { buf, topic }
    }
    pub fn write(&self, sample: D) {
        let _ = self.buf.send(sample);
    }
}


#[derive(Clone)]
// TODO: when deriving clone, this must update the forwarding thread. maybe have to add private reference to parent node?
pub struct Subscriber<D, T> {
    pub id: usize,
    buf: Receiver<D>,
    pub topic: T
}
impl<D, T> Subscriber<D, T> {
    pub fn new(id: usize, buf: Receiver<D>, topic: T) -> Self {
        Self { id, buf, topic }
    }
    
    
    pub fn read(&self) -> Option<D> {
        self.buf.read()
    }
    
    
    pub fn try_read(&self) -> Option<D> {
        self.buf.try_read()
    }
    
    
    pub fn read_blocking(&self) -> Option<D> {
        self.buf.read_blocking()
    }
    
    
    pub fn read_timeout(&self, timeout: std::time::Duration) -> Option<D> {
        self.buf.read_timeout(timeout)
    }


    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.buf.set_timeout(timeout);
    }
}

#[derive(Clone)]
/// Receiver
/// 
/// Define a receiver for subscribing messages
/// 
/// Defaultly reads from flume::Receiver, but also reads from crossbeam::channel::Receiver
/// when the number of publishers is greater than 1
pub struct Receiver<D> {
    buf: flume::Receiver<D>,
    plen: Arc<AtomicUsize>,
    pbuf: flume::Receiver<D>,
    timeout: std::time::Duration,
}
impl<D> Receiver<D> {
    pub fn new(
        buf: flume::Receiver<D>,
        plen: Arc<AtomicUsize>,
        pbuf: flume::Receiver<D>,
    ) -> Self {
        Self{ buf, plen, pbuf, timeout: std::time::Duration::from_millis(10) }
    }


    pub fn read(&self) -> Option<D> {
        self.read_timeout(self.timeout)
    }
    
    
    pub fn try_read(&self) -> Option<D> {
        match self.plen.load(Ordering::SeqCst) {
            0 => None,
            1 => self.buf.try_recv().ok(),
            _ => self.pbuf.try_recv().ok(),
        }
    }
    
    
    pub fn read_blocking(&self) -> Option<D> {
        match self.plen.load(Ordering::SeqCst) {
            0 => None,
            1 => self.buf.recv().ok(),
            _ => self.pbuf.recv().ok(),
        }
    }
    
    
    pub fn read_timeout(&self, timeout: std::time::Duration) -> Option<D> {
        match self.plen.load(Ordering::SeqCst) {
            0 => None,
            1 => self.buf.recv_timeout(timeout).ok(),
            _ => self.pbuf.recv_timeout(timeout).ok(),
        }
    }
    
    
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

pub trait PubSub<T> {
    fn publisher<D: 'static>(&mut self, topic: T) -> Result<Publisher<D, T>, Box<dyn std::error::Error>>;
    fn subscriber<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<Subscriber<D, T>, Box<dyn std::error::Error>>;
    fn pubsub<D: Clone + Send + 'static>(&mut self, topic: T) -> Result<(Publisher<D, T>, Subscriber<D, T>), Box<dyn std::error::Error>>;
    // fn participant<D: 'static>(&mut self, topic: T) -> Result<(), Box<dyn std::error::Error>>;
    fn delete_publisher<D: 'static>(&mut self, _publisher: Publisher<D, T>);
    fn delete_subscriber<D: Clone + Send + 'static>(&mut self, subscriber: Subscriber<D, T>);
}


pub fn forward<D: Clone>(
    i_buf: flume::Receiver<D>,
    o_bufs: Vec<flume::Sender<D>>,
    forwarding: Arc<AtomicBool>,
    terminate: flume::Receiver<usize>,
) {
    forwarding.store(true, Ordering::SeqCst);
    let num_publishers = o_bufs.len();
    tracing::debug!("Thread has spawned! Num distributors: {}", num_publishers);
    let mut exit_0 = false;
    let mut exit_1 = false;
    match num_publishers {
        0 => (),
        1 => {
            let publisher = &o_bufs[0];
            loop {
                let selector = flume::Selector::new()
                .recv(&i_buf, |sample| {
                    match sample {
                        Ok(sample) => { let _ = publisher.send(sample); },
                        Err(_) => exit_0 = true,
                    }
                })
                .recv(&terminate, |npub| {
                    match npub {
                        Ok(npub) => if npub == num_publishers { exit_1 = true; },
                        Err(_) => exit_1 = true,
                    }
                });
                selector.wait();
                if exit_0 || exit_1 { break; }
            }
        },
        _ => {
            let publisher_0 = &o_bufs[0];
            let publisher_n = &o_bufs[1..];
            loop {
                let selector = flume::Selector::new()
                .recv(&i_buf, |sample| {
                    match sample {
                        Ok(sample) => {
                            publisher_n
                            .iter()
                            .for_each(|publisher| { let _ = publisher.send(sample.clone()); });
                            let _ = publisher_0.send(sample);
                        },
                        Err(_) => exit_0 = true,
                    }
                })
                .recv(&terminate, |npub| {
                    match npub {
                        Ok(npub) => if npub == num_publishers { exit_1 = true; },
                        Err(_) => exit_1 = true,
                    }
                });
                selector.wait();
                if exit_0 || exit_1 { break; }
            }
        }
    }
    forwarding.store(false, Ordering::SeqCst);
}


pub fn downcast<T>(buf: Box<dyn Any + 'static>) -> Result<T, Box<dyn Any>>
where
    T: 'static,
{
    match buf.downcast::<T>() {
        Ok(t) => Ok(*t),
        Err(e) => Err(e),
    }
}

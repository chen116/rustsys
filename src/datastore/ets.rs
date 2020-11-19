use tokio::sync::{broadcast, Notify};
use tokio::time::{self, Duration, Instant};

use bytes::Bytes;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};




/// Server state shared across all connections.
///
/// `Db` contains a `HashMap` storing the key/value data and all
/// `broadcast::Sender` values for active pub/sub channels.
///
/// A `Db` instance is a handle to shared state. Cloning `Db` is shallow and
/// only incurs an atomic ref count increment.
///
/// When a `Db` value is created, a background task is spawned. This task is
/// used to expire values after the requested duration has elapsed. The task
/// runs until all instances of `Db` are dropped, at which point the task
/// terminates.
/// 
/// 
/// 
/// 
/// 
/// 
/// 
#[derive(Debug, Clone)]
pub struct SimpleEts {
    simpmap: Arc<Mutex<HashMap<String, String>>>,
}
pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
}
pub enum Response {
    Value {
        key: String,
        value: String,
    },
    Set {
        key: String,
        value: String,
        previous: Option<String>,
    },
    Error {
        msg: String,
    },
}
impl SimpleEts {
    pub fn new() -> SimpleEts {
        let shared = Arc::new( Mutex::new( HashMap::new()));
        SimpleEts { simpmap: shared }
    }

    pub fn handle_request(&self,line: &str) -> Response {
            let request = match Request::parse(&line) {
        Ok(req) => req,
        Err(e) => return Response::Error { msg: e },
    };
        // let state = self.shared.lock().unwrap();
        // state.entries.get(key).map(|entry| entry.data.clone())
    let mut state = self.simpmap.lock().unwrap();
    match request {
        Request::Get { key } => match state.get(&key) {
            Some(value) => Response::Value {
                key,
                value: value.clone(),
            },
            None => Response::Error {
                msg: format!("no key {}", key),
            },
        },
        Request::Set { key, value } => {
            let previous = state.insert(key.clone(), value.clone());
            Response::Set {
                key,
                value,
                previous,
            }
        }
        }
    }
}
impl Request {
    fn parse(input: &str) -> Result<Request, String> {
        let mut parts = input.splitn(3, ' ');
        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or("GET must be followed by a key")?;
                if parts.next().is_some() {
                    return Err("GET's key must not be followed by anything".into());
                }
                Ok(Request::Get {
                    key: key.to_string(),
                })
            }
            Some("SET") => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("SET must be followed by a key".into()),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("SET needs a value".into()),
                };
                Ok(Request::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some(cmd) => Err(format!("unknown command: {}", cmd)),
            None => Err("empty input".into()),
        }
    }
}
impl Response {
    pub fn serialize(&self) -> String {
        match *self {
            Response::Value { ref key, ref value } => format!("{} = {}", key, value),
            Response::Set {
                ref key,
                ref value,
                ref previous,
            } => format!("set {} = `{}`, previous: {:?}", key, value, previous),
            Response::Error { ref msg } => format!("error: {}", msg),
        }
    }
}



#[derive(Debug, Clone)]
pub struct Ets {
    /// Handle to shared state. The background task will also have an
    /// `Arc<Shared>`.
    shared: Arc<Mutex<Datastore>>,
}

// #[derive(Debug)]
// struct Shared {
//     /// The shared state is guarded by a mutex. This is a `std::sync::Mutex` and
//     /// not a Tokio mutex. This is because there are no asynchronous operations
//     /// being performed while holding the mutex. Additionally, the critical
//     /// sections are very small.
//     ///
//     /// A Tokio mutex is mostly intended to be used when locks need to be held
//     /// across `.await` yield points. All other cases are **usually** best
//     /// served by a std mutex. If the critical section does not include any
//     /// async operations but is long (CPU intensive or performing blocking
//     /// operations), then the entire operation, including waiting for the mutex,
//     /// is considered a "blocking" operation and `tokio::task::spawn_blocking`
//     /// should be used.
//     state: Mutex<State>,

//     /// Notifies the background task handling entry expiration. The background
//     /// task waits on this to be notified, then checks for expired values or the
//     /// shutdown signal.
//     background_task: Notify,
// }



#[derive(Debug)]
struct Datastore {
    /// The key-value data. We are not trying to do anything fancy so a
    /// `std::collections::HashMap` works fine.
    entries: HashMap<String, Entry>,
}

/// Entry in the key-value store
#[derive(Debug)]
struct Entry {
    /// Uniquely identifies this entry.
    id: u64,

    /// Stored data
    data: u64,

    bmap: BTreeMap<String, Vec<u64>>,

}

impl Ets {
    /// Create a new, empty, `Db` instance. Allocates shared state and spawns a
    /// background task to manage key expiration.
    pub fn new() -> Ets {


        let shared = Arc::new( Mutex::new( Datastore{
            entries: HashMap::new(),
        })


        ) ;
            



        Ets { shared: shared }
    }
    pub fn get(&self, key: &str) -> Option<u64> {
        // Acquire the lock, get the entry and clone the value.
        // Because data is stored using `Bytes`, a clone here is a shallow
        // clone. Data is not copied.
        let state = self.shared.lock().unwrap();
        state.entries.get(key).map(|entry| entry.data.clone())
    }

    pub fn getVec(&self, key: &str) -> Vec<u64> {
        // Acquire the lock, get the entry and clone the value.
        // Because data is stored using `Bytes`, a clone here is a shallow
        // clone. Data is not copied.
        let state = self.shared.lock().unwrap();
        let meow = match state.entries.get(key) {
            Some(entry) => {
                match entry.bmap.get(key) {
                    Some(vec) => vec,
                    None => panic!("ddd"),
                }
            },
            None => panic!("ddsd"),
        };
        meow.clone()
    }

    /// Set the value associated with a key along with an optional expiration
    /// Duration.
    ///
    /// If a value is already associated with the key, it is removed.
    pub fn set(&self, key: String, value: u64) {
        let mut state = self.shared.lock().unwrap();
        // let id = 1+state.entries.get(key.as_str()).map(|entry| entry.id.clone()).unwrap();
        // Insert the entry into the `HashMap`.
        let mut vec = Vec::new();
        vec.push(997);
        let mut map = BTreeMap::new();
        map.insert(key.clone(),vec);

        let prev = state.entries.insert(
            key,
            Entry {
                id: 2,
                data: value,
                bmap: map,
            },
        );
    }

    pub fn setVec(&self, key: String, metadata: u64) {
        let mut state = self.shared.lock().unwrap();
        let entry = state.entries.get_mut(key.as_str()).unwrap();
        if let Some(vec) = entry.bmap.get_mut(key.as_str()){
            vec.push(metadata);
        }
        // let mut vec = entry.bmap.get(key.as_str())

        // let meow = match state.entries.get(key.as_str()) {
        //     Some(entry) => {
        //         match entry.bmap.get(key.as_str()) {
        //             Some(vec) => vec,
        //             None => panic!("ddd"),
        //         }
        //     },
        //     None => panic!("ddsd"),
        // };
        // meow.clone()

    }

}


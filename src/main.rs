mod serialization;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync:: {
        atomic::{AtomicU64, Ordering},
        Arc, Mutex
    }
};
use lazy_static::lazy_static;

// Global incremental version number
static VERSION: AtomicU64 = AtomicU64::new(1);

fn acquire_next_version() -> u64 {
    let version = VERSION.fetch_add(1, Ordering::SeqCst);
    version
}
pub type KVEngine = BTreeMap<Vec<u8>, Option<Vec<u8>>>;

lazy_static! {
    static ref ACTIVE_TXN: Arc<Mutex<HashMap<u64, Vec<Vec<u8>>>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub struct MVCC {
    kv: Arc<Mutex<KVEngine>>,
}

impl MVCC {
    pub fn new(kv: KVEngine) -> Self {
        Self {
            kv: Arc::new(Mutex::new(kv)),
        }
    }
}

fn main() {

}

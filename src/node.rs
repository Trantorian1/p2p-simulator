use std::collections::HashMap;

use blake2::{Blake2b, Digest};
use rand::Rng;

pub type DATA = u8;
pub type GUID = [DATA; 30];
type Blake2b160 = Blake2b<blake2::digest::consts::U30>;

pub struct Node {
    guid: GUID,
    peers: Vec<Node>,
    storage: HashMap<GUID, DATA>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();
        let salt: GUID = rng.gen();

        let mut hasher = Blake2b160::new();
        hasher.update(salt);
        hasher.update(name.as_bytes());
        let guid: GUID = hasher.finalize().into();

        Self {
            guid,
            peers: vec![],
            storage: HashMap::default(),
        }
    }

    fn guid(&self) -> GUID {
        self.guid
    }

    fn peers(&self) -> &[Node] {
        &self.peers
    }

    fn relay(&self, data: &[DATA], id: GUID) -> Option<Vec<DATA>> {
        todo!()
    }

    fn ping(&self, id: GUID) -> bool {
        self.guid == id || self.storage.contains_key(&id)
    }
}

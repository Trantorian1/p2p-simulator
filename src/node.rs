use std::usize;

use blake2::{Blake2b, Digest};
use indexmap::IndexMap;
use rand::Rng;

pub type DATA = u8;
pub type GUID = [DATA; 30];
type Blake2b160 = Blake2b<blake2::digest::consts::U30>;

pub enum ConnectionStep {
    Done { data: Vec<DATA> },
    Failed { id: GUID },
    Seeking { id: GUID },
}

pub struct KBucket<const k: usize> {}

#[derive(Clone)]
pub struct Node {
    guid: GUID,
    peers: IndexMap<GUID, Node>,
    storage: IndexMap<GUID, DATA>,
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
            peers: IndexMap::default(),
            storage: IndexMap::default(),
        }
    }

    pub fn new_with_peers(name: &str, peers: &[Node]) -> Self {
        let mut rng = rand::thread_rng();
        let salt: GUID = rng.gen();

        let mut hasher = Blake2b160::new();
        hasher.update(salt);
        hasher.update(name.as_bytes());
        let guid: GUID = hasher.finalize().into();

        let peers = peers
            .iter()
            .cloned()
            .map(|node| (node.guid, node))
            .collect::<IndexMap<_, _>>();

        Self {
            guid,
            peers,
            storage: IndexMap::default(),
        }
    }

    fn guid(&self) -> GUID {
        self.guid
    }

    fn peers(&self) -> indexmap::map::Values<[u8; 30], Node> {
        self.peers.values()
    }

    fn query(&self, data: &[DATA], id: GUID) -> ConnectionStep {
        todo!()
    }

    fn store(&self, data: &[DATA], id: GUID) -> ConnectionStep {
        todo!()
    }

    fn find_next_peer(&self, id: GUID) -> ConnectionStep {
        todo!()
    }

    fn ping(&self, id: GUID) -> bool {
        todo!()
    }
}

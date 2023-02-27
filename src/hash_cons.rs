
use std::{collections::{BTreeMap, hash_map::DefaultHasher, btree_map::Entry}, hash::{Hash, Hasher}, mem::size_of};

use deepsize::DeepSizeOf;
use itertools::Itertools;

use crate::{packed_context::{ContextDelta, pack_context}, initial_context::Context, ContextSize};



#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, DeepSizeOf)]
pub struct ContextId(u64);


#[derive(Clone, Debug, Eq, PartialEq, Hash, Default, DeepSizeOf)]
pub struct SinglyLinkedList {
    head: Option<ContextDelta>,
    tail: Option<ContextId>,
}

pub struct ContextHash {
    hash: BTreeMap<ContextId, SinglyLinkedList>,
}

impl ContextHash {
    pub fn new() -> Self {
        ContextHash {
            hash: BTreeMap::new(),
        }
    }

    fn insert(&mut self, ctx: &Context) -> ContextId {
        let mut deltas = pack_context(ctx).iter().copied().collect_vec();
        deltas.reverse();
        deltas.push(ContextDelta::None);
        self.insert_internal(&deltas)
    }

    fn insert_internal(&mut self, deltas: &[ContextDelta]) -> ContextId {
        let deltas_hash = self.get_hash(deltas);

        match self.hash.entry(deltas_hash) {
            Entry::Occupied(_) => {
                assert!(deltas == self.get_deltas(deltas_hash));
            }
            Entry::Vacant(_) => {
                if deltas.len() == 1 {
                    self.hash.insert(deltas_hash, SinglyLinkedList {
                        head: Some(deltas[0]),
                        tail: None,
                    });
                } else {
                    let tail_hash = self.insert_internal(&deltas[1..]);
                    self.hash.insert(deltas_hash, SinglyLinkedList {
                        head: Some(deltas[0]),
                        tail: Some(tail_hash),
                    });
                }
            }
        }
        deltas_hash

    }

    fn get_deltas(&self, hash: ContextId) -> Vec<ContextDelta> {
        let mut deltas = vec![];
        let mut current_hash = hash;
        while let Some(link) = self.hash.get(&current_hash) {
            if let Some(head) = link.head {
                deltas.push(head);
            }
            if link.tail.is_none() {
                break;
            }
            current_hash = link.tail.unwrap();
        }
        deltas
    }

    fn get_hash(&mut self, deltas: &[ContextDelta]) -> ContextId {
        let mut hasher = DefaultHasher::new();
        for delta in deltas {
            delta.hash(&mut hasher);
        }
        ContextId(hasher.finish())
    }
}

impl Default for ContextHash {
    fn default() -> Self {
        Self::new()
    }
}



impl ContextSize for ContextHash {
    type Context = Context;
    type Pointer = ContextId;
    type Storage = BTreeMap<ContextId, SinglyLinkedList>;

    fn get_pointer(&self) -> ContextId {
        *self.hash.iter().next().unwrap().0
    }

    fn get_storage(&self) -> Option<BTreeMap<ContextId, SinglyLinkedList>> {
        Some(self.hash.clone())
    }

    fn get_pointer_size(&self, count: usize) -> usize {
        size_of::<Self::Pointer>() * count
    }

    fn store_context(&mut self, context: Self::Context) {
        self.insert(&context);
    }
}

use crate::data::log_record::LogRecordPos;
use crate::index::Indexer;
use parking_lot::RwLock;
use std::{collections::BTreeMap, sync::Arc};

pub struct BTree {
    tree: Arc<RwLock<BTreeMap<Vec<u8>, LogRecordPos>>>,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl Indexer for BTree {
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool {
        let mut write_guard = self.tree.write();
        write_guard.insert(key, pos);
        true
    }

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos> {
        let read_guard = self.tree.read();
        read_guard.get(&key).copied()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        let mut write_guard = self.tree.write();
        let removed = write_guard.remove(&key);
        removed.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::log_record::LogRecordPos;

    #[test]
    fn test_btree() {
        let btree = BTree::new();
        let key = vec![1, 2, 3];
        let pos = LogRecordPos {
            file_id: 1,
            offset: 2,
        };
        assert_eq!(btree.put(key.clone(), pos), true);
        assert_eq!(btree.get(key.clone()), Some(pos));
        assert_eq!(btree.delete(key.clone()), true);
        assert_eq!(btree.get(key.clone()), None);
    }

    #[test]
    fn test_btree_put() {
        let btree = BTree::new();
        let key = vec![1, 2, 3];
        let pos = LogRecordPos {
            file_id: 1,
            offset: 2,
        };
        assert_eq!(btree.put(key.clone(), pos), true);
        let ket = "".as_bytes().to_vec();
        let pos = LogRecordPos {
            file_id: 1,
            offset: 2,
        };
        assert_eq!(btree.put(ket.clone(), pos), true);
    }

    #[test]
    fn test_btree_get() {
        let btree = BTree::new();
        let key = vec![1, 2, 3];
        let pos = LogRecordPos {
            file_id: 1,
            offset: 2,
        };
        assert_eq!(btree.get(key.clone()), None);
        btree.put(key.clone(), pos);
        assert_eq!(btree.get(key.clone()), Some(pos));
    }

    #[test]
    fn test_btree_delete() {
        let btree = BTree::new();
        let key = vec![1, 2, 3];
        let pos = LogRecordPos {
            file_id: 1,
            offset: 2,
        };
        assert_eq!(btree.delete(key.clone()), false);
        btree.put(key.clone(), pos);
        assert_eq!(btree.delete(key.clone()), true);
        assert_eq!(btree.delete(key.clone()), false);
    }
}

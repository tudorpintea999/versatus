use lr_trie::{Bytes, LeftRightTrie};
use patriecia::{db::Database, H256};
use std::{fmt::Debug, sync::Arc};

pub struct TxTrie<'a, D: Database> {
    trie: LeftRightTrie<'a, D>,
}

impl<'a, D: Database> TxTrie<'a, D> {
    /// Creates a new empty state trie.
    pub fn new(db: Arc<D>) -> Self {
        Self {
            trie: LeftRightTrie::new(db),
        }
    }

    /// Adds a single leaf value serialized to bytes
    /// Example:
    /// ```
    ///  use tx_trie::TxTrie;
    ///  use std::sync::Arc;
    ///  use patriecia::db::MemoryDB;
    ///
    ///  let memdb = Arc::new(MemoryDB::new(true));
    ///  let mut tx_trie = TxTrie::new(memdb);
    ///  
    ///  tx_trie.add(b"greetings", b"hello world");
    ///
    ///  assert_eq!(tx_trie.len(), 1);
    /// ```
    ///
    pub fn add(&mut self, key: &'a Bytes, value: &'a Bytes) {
        self.trie.add(key, value);
    }

    /// Extends the state trie with the provided iterator over leaf values as bytes.
    /// Example:
    /// ```
    ///  use tx_trie::TxTrie;
    ///  use std::sync::Arc;
    ///  use lr_trie::Bytes;
    ///  use patriecia::db::MemoryDB;
    ///
    ///  let memdb = Arc::new(MemoryDB::new(true));
    ///  let mut tx_trie = TxTrie::new(memdb);
    ///
    ///  let vals: Vec<(&Bytes, &Bytes)> = vec![
    ///      (b"abcdefg", b"abcdefg"),
    ///      (b"hijkl", b"hijkl"),
    ///      (b"mnopq", b"mnopq"),
    ///  ];
    ///
    ///  tx_trie.extend(vals);
    ///  assert_eq!(tx_trie.len(), 2);
    /// ```
    ///
    pub fn extend(&mut self, values: Vec<(&'a Bytes, &'a Bytes)>) {
        self.trie.extend(values);
    }

    /// Returns the trie's Merkle root.
    /// Example:
    /// ```
    ///  use tx_trie::TxTrie;
    ///  use std::sync::Arc;
    ///  use lr_trie::Bytes;
    ///  use patriecia::db::MemoryDB;
    ///
    ///  let memdb = Arc::new(MemoryDB::new(true));
    ///  let mut tx_trie_a = TxTrie::new(memdb);
    ///
    ///  let memdb = Arc::new(MemoryDB::new(true));
    ///  let mut tx_trie_b = TxTrie::new(memdb);
    ///
    ///  let vals: Vec<(&Bytes, &Bytes)> = vec![
    ///      (b"abcdefg", b"abcdefg"),
    ///      (b"hijkl", b"hijkl"),
    ///      (b"mnopq", b"mnopq"),
    ///  ];
    ///
    ///  tx_trie_a.extend(vals.clone());
    ///  tx_trie_b.extend(vals.clone());
    ///
    ///  assert_eq!(tx_trie_a.root(), tx_trie_b.root());
    /// ```
    ///
    pub fn root(&self) -> Option<H256> {
        self.trie.root()
    }

    /// Returns the count of leaves in the state trie.
    /// Example:
    /// ```
    ///  use tx_trie::TxTrie;
    ///  use std::sync::Arc;
    ///  use lr_trie::Bytes;
    ///  use patriecia::db::MemoryDB;
    ///
    ///  let memdb = Arc::new(MemoryDB::new(true));
    ///  let mut tx_trie = TxTrie::new(memdb);
    ///
    ///  let vals: Vec<(&Bytes, &Bytes)> = vec![
    ///      (b"abcdefg", b"abcdefg"),
    ///      (b"hijkl", b"hijkl"),
    ///      (b"mnopq", b"mnopq"),
    ///  ];
    ///
    ///  tx_trie.extend(vals);
    ///
    ///  assert_eq!(tx_trie.len(), 2);
    /// ```
    ///
    pub fn len(&self) -> usize {
        self.trie.len()
    }

    /// Returns true if there are no values in the trie.
    /// Example:
    /// ```
    ///  use tx_trie::TxTrie;
    ///  use std::sync::Arc;
    ///  use patriecia::db::MemoryDB;
    ///
    ///  let memdb = Arc::new(MemoryDB::new(true));
    ///  let mut tx_trie = TxTrie::new(memdb);
    ///
    ///  assert_eq!(tx_trie.len(), 0);
    /// ```
    ///
    pub fn is_empty(&self) -> bool {
        self.trie.len() == 0
    }
}

impl<'a, D: Database> PartialEq for TxTrie<'a, D> {
    fn eq(&self, other: &Self) -> bool {
        self.root() == other.root()
    }
}

impl<'a, D: Database> Default for TxTrie<'a, D> {
    fn default() -> Self {
        Self {
            trie: Default::default(),
        }
    }
}

impl<'a, D: Database> Debug for TxTrie<'a, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: derive once MerkleTree impl Debug
        f.debug_struct("TxTrie").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use patriecia::db::MemoryDB;
    use std::sync::Arc;

    #[test]
    fn new_creates_default_empty_trie() {
        let memdb = Arc::new(MemoryDB::new(true));
        let tx_trie = TxTrie::new(memdb);

        assert!(tx_trie.root().is_some());
        assert_eq!(tx_trie.len(), 1);
    }

    #[test]
    fn new_creates_trie_from_lrdb_values() {
        let memdb = Arc::new(MemoryDB::new(true));
        let mut tx_trie = TxTrie::new(memdb);

        tx_trie.add(b"abcdefg", b"12345");
        tx_trie.add(b"hijkl", b"1000");
        tx_trie.add(b"mnopq", b"askskaskj");

        let root = tx_trie.root().unwrap();
        let root = format!("0x{}", hex::encode(root));

        let target_root =
            "0xfcea4ea8a4decaf828666306c81977085ba9488d981c759ac899862fd4e9174e".to_string();

        assert_eq!(tx_trie.len(), 4);
        assert_eq!(root, target_root);
    }

    #[test]
    fn should_add_node_to_trie() {
        let memdb = Arc::new(MemoryDB::new(true));
        let mut tx_trie = TxTrie::new(memdb);

        assert!(tx_trie.root().is_some());
        assert_eq!(tx_trie.len(), 1);

        tx_trie.add(b"greetings", b"hello world");

        assert_ne!(tx_trie.root(), None);
        assert_eq!(tx_trie.len(), 2);
    }

    #[test]
    fn should_extend_trie_with_nodes() {
        let memdb = Arc::new(MemoryDB::new(true));
        let mut tx_trie = TxTrie::new(memdb);

        assert!(tx_trie.root().is_some());
        assert_eq!(tx_trie.len(), 1);

        let vals: Vec<(&Bytes, &Bytes)> = vec![
            (b"abcdefg", b"abcdefg"),
            (b"hijkl", b"hijkl"),
            (b"mnopq", b"mnopq"),
        ];

        tx_trie.extend(vals);

        assert_ne!(tx_trie.root(), None);
        assert_eq!(tx_trie.len(), 3);
    }

    #[test]
    fn should_return_true_if_root_is_equal_to_other_trie_root() {
        let memdb = Arc::new(MemoryDB::new(true));

        let mut tx_trie_a = TxTrie::new(memdb.clone());
        let mut tx_trie_b = TxTrie::new(memdb);

        let vals: Vec<(&Bytes, &Bytes)> = vec![
            (b"abcdefg", b"abcdefg"),
            (b"hijkl", b"hijkl"),
            (b"mnopq", b"mnopq"),
        ];

        tx_trie_a.extend(vals.clone());
        tx_trie_b.extend(vals.clone());

        assert_eq!(tx_trie_a, tx_trie_b);
    }

    #[test]
    fn should_return_false_if_root_is_not_equal_to_other_trie_root() {
        let memdb = Arc::new(MemoryDB::new(true));

        let mut tx_trie_a = TxTrie::new(memdb.clone());
        let mut tx_trie_b = TxTrie::new(memdb.clone());

        let vals: Vec<(&Bytes, &Bytes)> = vec![
            (b"abcdefg", b"abcdefg"),
            (b"hijkl", b"hijkl"),
            (b"mnopq", b"mnopq"),
        ];

        tx_trie_a.extend(vals.clone());
        tx_trie_b.extend(vals.clone());
        tx_trie_b.add(b"mnopq", b"bananas");

        assert_ne!(tx_trie_a, tx_trie_b);
    }
}

// TODO: revisit later once lrdb is integrated with tries
// impl<'a, D, E> From<E> for TxTrie<'a, D>
// where
//     D: Database,
//     E: Iterator<Item = &'a Bytes>,
// {
//     fn from(values: E) -> Self {
//         let trie = LeftRightTrie::from(values);
//         Self { trie }
//     }
// }
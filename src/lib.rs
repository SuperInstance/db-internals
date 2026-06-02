//! # db-internals
//!
//! Database internals in Rust — B-trees, hash indexes, WAL, ARIES recovery,
//! query optimization, ACID transactions, concurrency control, and normal forms.
//!
//! Learn databases by reading Rust.

pub mod relational;
pub mod btree;
pub mod hash_index;
pub mod query_opt;
pub mod transactions;
pub mod concurrency;
pub mod recovery;
pub mod normal_forms;

pub use relational::*;
pub use btree::BTree;
pub use hash_index::{ChainedHashTable, ExtendibleHashTable};
pub use query_opt::QueryOptimizer;
pub use transactions::*;
pub use concurrency::*;
pub use recovery::*;
pub use normal_forms::*;

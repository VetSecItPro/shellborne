//! Bounded collections (layer 2, plan step P0.10.7).
//!
//! The only constructor of each type takes both a capacity and an eviction policy; the types have no
//! `Default` impl, no zero-argument constructor and no public fields. The cap is a count of items for
//! [`BoundedQueue`], [`BoundedMap`], [`BoundedSet`] and [`BoundedList`], and a count of bytes for
//! [`ByteRing`]. A zero cap is refused at construction with [`ZeroCap`]. Storage grows as items
//! arrive; building a collection with a large cap does not reserve that much memory up front.
//!
//! A cap and a rule are required to build any of them:
//!
//! ```
//! use sb_bounds::{BoundedList, BoundedMap, BoundedQueue, BoundedSet, ByteRing, Eviction, ZeroCap};
//! # fn main() -> Result<(), ZeroCap> {
//! let _queue = BoundedQueue::<u8>::new(4, Eviction::RefuseNew)?;
//! let _map = BoundedMap::<u8, u8>::new(4, Eviction::EvictOldest)?;
//! let _set = BoundedSet::<u8>::new(4, Eviction::RefuseNew)?;
//! let _list = BoundedList::<u8>::new(4, Eviction::EvictOldest)?;
//! let _ring = ByteRing::new(4, Eviction::RefuseNew)?;
//! # Ok(())
//! # }
//! ```
//!
//! There is no default value for any of them:
//!
//! ```compile_fail
//! let _queue: sb_bounds::BoundedQueue<u8> = Default::default();
//! ```
//!
//! ```compile_fail
//! let _map: sb_bounds::BoundedMap<u8, u8> = Default::default();
//! ```
//!
//! ```compile_fail
//! let _set: sb_bounds::BoundedSet<u8> = Default::default();
//! ```
//!
//! ```compile_fail
//! let _list: sb_bounds::BoundedList<u8> = Default::default();
//! ```
//!
//! ```compile_fail
//! let _ring: sb_bounds::ByteRing = Default::default();
//! ```
//!
//! And no constructor without a cap:
//!
//! ```compile_fail
//! let _queue = sb_bounds::BoundedQueue::<u8>::new();
//! ```
//!
//! ```compile_fail
//! let _map = sb_bounds::BoundedMap::<u8, u8>::new();
//! ```
//!
//! ```compile_fail
//! let _set = sb_bounds::BoundedSet::<u8>::new();
//! ```
//!
//! ```compile_fail
//! let _list = sb_bounds::BoundedList::<u8>::new();
//! ```
//!
//! ```compile_fail
//! let _ring = sb_bounds::ByteRing::new();
//! ```
//!
//! A cap alone, without an eviction rule, does not build any of them either:
//!
//! ```compile_fail
//! let _queue = sb_bounds::BoundedQueue::<u8>::new(4);
//! ```
//!
//! ```compile_fail
//! let _ring = sb_bounds::ByteRing::new(4);
//! ```

#![expect(
    clippy::disallowed_types,
    reason = "the one crate that may use the std collections: it wraps each behind a cap and an eviction rule"
)]

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::hash::{BuildHasher, Hash, RandomState};

/// What a full collection does with a new item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eviction {
    /// Keep what is stored and return the rejected item.
    RefuseNew,
    /// Evict the oldest item by insertion order and hand the evicted item back.
    EvictOldest,
}

/// A zero cap was given to a constructor. Every cap must be at least one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroCap;

impl fmt::Display for ZeroCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a bounded collection needs a cap of at least one")
    }
}

impl std::error::Error for ZeroCap {}

/// Refuses a zero cap.
fn checked_cap(cap: usize) -> Result<usize, ZeroCap> {
    if cap == 0 { Err(ZeroCap) } else { Ok(cap) }
}

/// Adds `count` to an eviction counter, stopping at the largest value rather than wrapping.
fn count_evictions(counter: &mut u64, count: usize) {
    let count = u64::try_from(count).unwrap_or(u64::MAX);
    *counter = counter.saturating_add(count);
}

/// The shared first-in, first-out store behind [`BoundedQueue`] and [`BoundedList`].
#[derive(Debug)]
struct Line<T> {
    cap: usize,
    eviction: Eviction,
    evictions: u64,
    items: VecDeque<T>,
}

impl<T> Line<T> {
    fn new(cap: usize, eviction: Eviction) -> Result<Self, ZeroCap> {
        Ok(Self {
            cap: checked_cap(cap)?,
            eviction,
            evictions: 0,
            items: VecDeque::new(),
        })
    }

    fn push(&mut self, item: T) -> Result<Option<T>, T> {
        let evicted = if self.items.len() >= self.cap {
            match self.eviction {
                Eviction::RefuseNew => return Err(item),
                Eviction::EvictOldest => {
                    let oldest = self.items.pop_front();
                    if oldest.is_some() {
                        count_evictions(&mut self.evictions, 1);
                    }
                    oldest
                }
            }
        } else {
            None
        };
        self.items.push_back(item);
        Ok(evicted)
    }
}

/// A first-in, first-out queue that holds at most `cap` items.
///
/// [`BoundedQueue::push`] returns `Ok(None)` when the item went in with room to spare,
/// `Ok(Some(oldest))` when it went in by evicting the oldest item, and `Err(item)` when the queue is
/// full under [`Eviction::RefuseNew`].
#[derive(Debug)]
pub struct BoundedQueue<T> {
    line: Line<T>,
}

impl<T> BoundedQueue<T> {
    /// Builds an empty queue that holds at most `cap` items. A zero cap is refused.
    pub fn new(cap: usize, eviction: Eviction) -> Result<Self, ZeroCap> {
        Ok(Self {
            line: Line::new(cap, eviction)?,
        })
    }

    /// Adds an item at the back.
    pub fn push(&mut self, item: T) -> Result<Option<T>, T> {
        self.line.push(item)
    }

    /// Removes and returns the oldest item.
    pub fn pop(&mut self) -> Option<T> {
        self.line.items.pop_front()
    }

    /// The number of items held.
    pub fn len(&self) -> usize {
        self.line.items.len()
    }

    /// Whether the queue holds no items.
    pub fn is_empty(&self) -> bool {
        self.line.items.is_empty()
    }

    /// The most items the queue will hold.
    pub fn cap(&self) -> usize {
        self.line.cap
    }

    /// How many items have been evicted since construction.
    pub fn evictions(&self) -> u64 {
        self.line.evictions
    }

    /// How many items the internal storage has room for right now. For tests of the growth rule.
    #[doc(hidden)]
    pub fn allocated(&self) -> usize {
        self.line.items.capacity()
    }
}

/// One entry of an [`Ordered`] store, linked to its neighbors in insertion order.
#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    hash: u64,
    older: Option<usize>,
    newer: Option<usize>,
}

/// The shared keyed store behind [`BoundedMap`] and [`BoundedSet`].
///
/// Each key is held once, in a slot of `slots`. The slots are linked oldest to newest, and `index`
/// finds a key's slot by its hash. A freed slot is reused before the slots grow, so the number of
/// slots never passes the most entries held at once, which is at most the cap.
#[derive(Debug)]
struct Ordered<K, V> {
    cap: usize,
    eviction: Eviction,
    evictions: u64,
    len: usize,
    slots: Vec<Option<Node<K, V>>>,
    free: Vec<usize>,
    index: HashMap<u64, Vec<usize>>,
    hasher: RandomState,
    oldest: Option<usize>,
    newest: Option<usize>,
}

impl<K: Eq + Hash, V> Ordered<K, V> {
    fn new(cap: usize, eviction: Eviction) -> Result<Self, ZeroCap> {
        Ok(Self {
            cap: checked_cap(cap)?,
            eviction,
            evictions: 0,
            len: 0,
            slots: Vec::new(),
            free: Vec::new(),
            index: HashMap::new(),
            hasher: RandomState::new(),
            oldest: None,
            newest: None,
        })
    }

    fn node(&self, slot: usize) -> Option<&Node<K, V>> {
        self.slots.get(slot).and_then(Option::as_ref)
    }

    fn node_mut(&mut self, slot: usize) -> Option<&mut Node<K, V>> {
        self.slots.get_mut(slot).and_then(Option::as_mut)
    }

    /// The slot holding `key`, if present.
    fn locate(&self, key: &K) -> Option<usize> {
        let hash = self.hasher.hash_one(key);
        self.index.get(&hash)?.iter().copied().find(|&slot| {
            self.node(slot)
                .is_some_and(|node| node.hash == hash && node.key == *key)
        })
    }

    fn get(&self, key: &K) -> Option<&V> {
        let slot = self.locate(key)?;
        self.node(slot).map(|node| &node.value)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let slot = self.locate(key)?;
        self.node_mut(slot).map(|node| &mut node.value)
    }

    /// Takes the entry in `slot` out of the order, the index and the slots.
    fn unlink(&mut self, slot: usize) -> Option<(K, V)> {
        let node = self.slots.get_mut(slot)?.take()?;
        match node.older {
            Some(older) => {
                if let Some(older) = self.node_mut(older) {
                    older.newer = node.newer;
                }
            }
            None => self.oldest = node.newer,
        }
        match node.newer {
            Some(newer) => {
                if let Some(newer) = self.node_mut(newer) {
                    newer.older = node.older;
                }
            }
            None => self.newest = node.older,
        }
        if let Some(bucket) = self.index.get_mut(&node.hash) {
            bucket.retain(|&held| held != slot);
            if bucket.is_empty() {
                self.index.remove(&node.hash);
            }
        }
        self.free.push(slot);
        self.len = self.len.saturating_sub(1);
        Some((node.key, node.value))
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let slot = self.locate(key)?;
        self.unlink(slot).map(|(_, value)| value)
    }

    /// Adds an entry whose key the caller has checked is absent, as the newest entry.
    fn push_new(&mut self, key: K, value: V) -> Result<Option<(K, V)>, (K, V)> {
        let evicted = if self.len >= self.cap {
            match self.eviction {
                Eviction::RefuseNew => return Err((key, value)),
                Eviction::EvictOldest => {
                    let oldest = self.oldest.and_then(|slot| self.unlink(slot));
                    if oldest.is_some() {
                        count_evictions(&mut self.evictions, 1);
                    }
                    oldest
                }
            }
        } else {
            None
        };

        let hash = self.hasher.hash_one(&key);
        let node = Node {
            key,
            value,
            hash,
            older: self.newest,
            newer: None,
        };
        let slot = match self.free.pop() {
            Some(slot) if slot < self.slots.len() => {
                if let Some(free) = self.slots.get_mut(slot) {
                    *free = Some(node);
                }
                slot
            }
            _ => {
                let slot = self.slots.len();
                self.slots.push(Some(node));
                slot
            }
        };
        match self.newest {
            Some(newest) => {
                if let Some(newest) = self.node_mut(newest) {
                    newest.newer = Some(slot);
                }
            }
            None => self.oldest = Some(slot),
        }
        self.newest = Some(slot);
        self.index.entry(hash).or_default().push(slot);
        self.len = self.len.saturating_add(1);
        Ok(evicted)
    }

    fn allocated(&self) -> usize {
        self.slots.capacity().max(self.index.capacity())
    }
}

/// What happened when an entry went into a [`BoundedMap`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapInsert<K, V> {
    /// A new key went in with room to spare.
    Added,
    /// The key was already present; its value was replaced and the old value is returned. A
    /// replacement is never an eviction and keeps the key's place in insertion order.
    Replaced(V),
    /// A new key went in by evicting the oldest entry, which is returned.
    Evicted(K, V),
}

/// A map that holds at most `cap` entries.
///
/// [`BoundedMap::insert`] returns `Err((key, value))` when a new key arrives at the cap under
/// [`Eviction::RefuseNew`]. Replacing the value of a present key always succeeds.
#[derive(Debug)]
pub struct BoundedMap<K, V> {
    store: Ordered<K, V>,
}

impl<K: Eq + Hash, V> BoundedMap<K, V> {
    /// Builds an empty map that holds at most `cap` entries. A zero cap is refused.
    pub fn new(cap: usize, eviction: Eviction) -> Result<Self, ZeroCap> {
        Ok(Self {
            store: Ordered::new(cap, eviction)?,
        })
    }

    /// Inserts an entry, or replaces the value of a present key.
    pub fn insert(&mut self, key: K, value: V) -> Result<MapInsert<K, V>, (K, V)> {
        if let Some(held) = self.store.get_mut(&key) {
            return Ok(MapInsert::Replaced(std::mem::replace(held, value)));
        }
        Ok(match self.store.push_new(key, value)? {
            Some((key, value)) => MapInsert::Evicted(key, value),
            None => MapInsert::Added,
        })
    }

    /// The value for `key`, if present.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }

    /// Whether `key` is present.
    pub fn contains_key(&self, key: &K) -> bool {
        self.store.locate(key).is_some()
    }

    /// Removes `key` and returns its value. A removal is not an eviction.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key)
    }

    /// The number of entries held.
    pub fn len(&self) -> usize {
        self.store.len
    }

    /// Whether the map holds no entries.
    pub fn is_empty(&self) -> bool {
        self.store.len == 0
    }

    /// The most entries the map will hold.
    pub fn cap(&self) -> usize {
        self.store.cap
    }

    /// How many entries have been evicted since construction.
    pub fn evictions(&self) -> u64 {
        self.store.evictions
    }

    /// How many entries the internal storage has room for right now. For tests of the growth rule.
    #[doc(hidden)]
    pub fn allocated(&self) -> usize {
        self.store.allocated()
    }
}

/// What happened when an item went into a [`BoundedSet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetInsert<T> {
    /// A new item went in with room to spare.
    Added,
    /// The item was already present. Nothing changed: no eviction, and its place in insertion order
    /// is kept.
    AlreadyPresent,
    /// A new item went in by evicting the oldest item, which is returned.
    Evicted(T),
}

/// A set that holds at most `cap` items.
///
/// [`BoundedSet::insert`] returns `Err(item)` when a new item arrives at the cap under
/// [`Eviction::RefuseNew`]. Inserting an item already present always succeeds.
#[derive(Debug)]
pub struct BoundedSet<T> {
    store: Ordered<T, ()>,
}

impl<T: Eq + Hash> BoundedSet<T> {
    /// Builds an empty set that holds at most `cap` items. A zero cap is refused.
    pub fn new(cap: usize, eviction: Eviction) -> Result<Self, ZeroCap> {
        Ok(Self {
            store: Ordered::new(cap, eviction)?,
        })
    }

    /// Inserts an item.
    pub fn insert(&mut self, item: T) -> Result<SetInsert<T>, T> {
        if self.store.locate(&item).is_some() {
            return Ok(SetInsert::AlreadyPresent);
        }
        match self.store.push_new(item, ()) {
            Ok(Some((evicted, ()))) => Ok(SetInsert::Evicted(evicted)),
            Ok(None) => Ok(SetInsert::Added),
            Err((item, ())) => Err(item),
        }
    }

    /// Whether `item` is present.
    pub fn contains(&self, item: &T) -> bool {
        self.store.locate(item).is_some()
    }

    /// Removes `item` and reports whether it was present. A removal is not an eviction.
    pub fn remove(&mut self, item: &T) -> bool {
        self.store.remove(item).is_some()
    }

    /// The number of items held.
    pub fn len(&self) -> usize {
        self.store.len
    }

    /// Whether the set holds no items.
    pub fn is_empty(&self) -> bool {
        self.store.len == 0
    }

    /// The most items the set will hold.
    pub fn cap(&self) -> usize {
        self.store.cap
    }

    /// How many items have been evicted since construction.
    pub fn evictions(&self) -> u64 {
        self.store.evictions
    }

    /// How many items the internal storage has room for right now. For tests of the growth rule.
    #[doc(hidden)]
    pub fn allocated(&self) -> usize {
        self.store.allocated()
    }
}

/// An ordered list that holds at most `cap` items, read by position, oldest first.
///
/// [`BoundedList::push`] has the same results as [`BoundedQueue::push`].
#[derive(Debug)]
pub struct BoundedList<T> {
    line: Line<T>,
}

impl<T> BoundedList<T> {
    /// Builds an empty list that holds at most `cap` items. A zero cap is refused.
    pub fn new(cap: usize, eviction: Eviction) -> Result<Self, ZeroCap> {
        Ok(Self {
            line: Line::new(cap, eviction)?,
        })
    }

    /// Adds an item at the end.
    pub fn push(&mut self, item: T) -> Result<Option<T>, T> {
        self.line.push(item)
    }

    /// The item at `index`, counting from the oldest held item.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.line.items.get(index)
    }

    /// The held items, oldest first.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.line.items.iter()
    }

    /// The number of items held.
    pub fn len(&self) -> usize {
        self.line.items.len()
    }

    /// Whether the list holds no items.
    pub fn is_empty(&self) -> bool {
        self.line.items.is_empty()
    }

    /// The most items the list will hold.
    pub fn cap(&self) -> usize {
        self.line.cap
    }

    /// How many items have been evicted since construction.
    pub fn evictions(&self) -> u64 {
        self.line.evictions
    }

    /// How many items the internal storage has room for right now. For tests of the growth rule.
    #[doc(hidden)]
    pub fn allocated(&self) -> usize {
        self.line.items.capacity()
    }
}

/// A write to a [`ByteRing`] under [`Eviction::RefuseNew`] that did not fit.
///
/// The ring stored the first `accepted` bytes of the write and none of the remaining `rejected` bytes.
/// A refusal is not an eviction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RingRefused {
    /// How many bytes of the write, from its start, the ring stored.
    pub accepted: usize,
    /// How many bytes at the end of the write the ring did not store.
    pub rejected: usize,
}

impl fmt::Display for RingRefused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a full byte ring stored {} bytes of a write and refused the other {}",
            self.accepted, self.rejected
        )
    }
}

impl std::error::Error for RingRefused {}

/// A byte buffer that holds at most `cap` bytes, oldest first.
///
/// Under [`Eviction::EvictOldest`] it holds the newest `cap` bytes written to it: a write that would
/// pass the cap evicts the oldest bytes, a single write longer than the cap keeps only its last `cap`
/// bytes, and [`ByteRing::push`] never fails. Under [`Eviction::RefuseNew`] it holds the first `cap`
/// bytes written to it: a write stores only the bytes that fit and returns [`RingRefused`] for the
/// rest, so what is held is always an unbroken prefix of everything written.
#[derive(Debug)]
pub struct ByteRing {
    cap: usize,
    eviction: Eviction,
    evictions: u64,
    bytes: VecDeque<u8>,
}

impl ByteRing {
    /// Builds an empty ring that holds at most `cap` bytes. A zero cap is refused.
    pub fn new(cap: usize, eviction: Eviction) -> Result<Self, ZeroCap> {
        Ok(Self {
            cap: checked_cap(cap)?,
            eviction,
            evictions: 0,
            bytes: VecDeque::new(),
        })
    }

    /// Appends `bytes`.
    ///
    /// Returns `Ok(evicted)` when every byte of the write was stored, where `evicted` counts the
    /// bytes this write evicted under [`Eviction::EvictOldest`]: both older bytes pushed out and
    /// bytes of this write that never fit. It is always zero under [`Eviction::RefuseNew`]. Returns
    /// `Err(RingRefused)` under [`Eviction::RefuseNew`] when the write did not fit; the bytes that fit
    /// were stored and the rest were not.
    pub fn push(&mut self, bytes: &[u8]) -> Result<usize, RingRefused> {
        match self.eviction {
            Eviction::RefuseNew => self.push_refusing(bytes),
            Eviction::EvictOldest => Ok(self.push_evicting(bytes)),
        }
    }

    fn push_refusing(&mut self, bytes: &[u8]) -> Result<usize, RingRefused> {
        let room = self.cap.saturating_sub(self.bytes.len());
        let accepted = bytes.len().min(room);
        let (stored, refused) = bytes.split_at_checked(accepted).unwrap_or((bytes, &[]));
        self.bytes.extend(stored);
        if refused.is_empty() {
            Ok(0)
        } else {
            Err(RingRefused {
                accepted: stored.len(),
                rejected: refused.len(),
            })
        }
    }

    fn push_evicting(&mut self, bytes: &[u8]) -> usize {
        let evicted = if bytes.len() >= self.cap {
            let never_fit = bytes.len().saturating_sub(self.cap);
            let evicted = self.bytes.len().saturating_add(never_fit);
            self.bytes.clear();
            self.bytes
                .extend(bytes.get(never_fit..).unwrap_or_default());
            evicted
        } else {
            let over = self
                .bytes
                .len()
                .saturating_add(bytes.len())
                .saturating_sub(self.cap)
                .min(self.bytes.len());
            self.bytes.drain(..over);
            self.bytes.extend(bytes);
            over
        };
        count_evictions(&mut self.evictions, evicted);
        evicted
    }

    /// A copy of the held bytes, oldest first.
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.iter().copied().collect()
    }

    /// The number of bytes held.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the ring holds no bytes.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The most bytes the ring will hold.
    pub fn cap(&self) -> usize {
        self.cap
    }

    /// How many bytes have been evicted since construction.
    pub fn evictions(&self) -> u64 {
        self.evictions
    }

    /// How many bytes the internal storage has room for right now. For tests of the growth rule.
    #[doc(hidden)]
    pub fn allocated(&self) -> usize {
        self.bytes.capacity()
    }
}

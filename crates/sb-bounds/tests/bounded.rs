//! Behavior of the bounded collections (plan step P0.10.7): a zero cap is refused, a full collection
//! either refuses the new item or evicts the oldest, every eviction is counted, and storage is not
//! reserved up front.

use sb_bounds::{
    BoundedList, BoundedMap, BoundedQueue, BoundedSet, ByteRing, Eviction, MapInsert, SetInsert,
    ZeroCap,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// A cap far larger than any test fills. Building with it must not reserve room for it.
const LARGE_CAP: usize = 1 << 24;

/// The most room the storage may hold after construction or a few inserts with [`LARGE_CAP`].
const SMALL_ALLOCATION: usize = 64;

const RULES: [Eviction; 2] = [Eviction::RefuseNew, Eviction::EvictOldest];

// Zero cap

#[test]
fn zero_cap_is_refused_for_every_type() {
    for rule in RULES {
        assert_eq!(BoundedQueue::<u8>::new(0, rule).err(), Some(ZeroCap));
        assert_eq!(BoundedMap::<u8, u8>::new(0, rule).err(), Some(ZeroCap));
        assert_eq!(BoundedSet::<u8>::new(0, rule).err(), Some(ZeroCap));
        assert_eq!(BoundedList::<u8>::new(0, rule).err(), Some(ZeroCap));
        assert_eq!(ByteRing::new(0, rule).err(), Some(ZeroCap));
    }
}

#[test]
fn cap_of_one_is_accepted_and_starts_empty() -> TestResult {
    for rule in RULES {
        let queue = BoundedQueue::<u8>::new(1, rule)?;
        assert_eq!((queue.cap(), queue.len(), queue.is_empty()), (1, 0, true));
        assert_eq!(queue.evictions(), 0);

        let map = BoundedMap::<u8, u8>::new(1, rule)?;
        assert_eq!((map.cap(), map.len(), map.is_empty()), (1, 0, true));
        assert_eq!(map.evictions(), 0);

        let set = BoundedSet::<u8>::new(1, rule)?;
        assert_eq!((set.cap(), set.len(), set.is_empty()), (1, 0, true));
        assert_eq!(set.evictions(), 0);

        let list = BoundedList::<u8>::new(1, rule)?;
        assert_eq!((list.cap(), list.len(), list.is_empty()), (1, 0, true));
        assert_eq!(list.evictions(), 0);

        let ring = ByteRing::new(1, rule)?;
        assert_eq!((ring.cap(), ring.len(), ring.is_empty()), (1, 0, true));
        assert_eq!(ring.evictions(), 0);
    }
    Ok(())
}

// Queue

#[test]
fn queue_refuse_new_hands_back_the_item_at_the_cap() -> TestResult {
    let mut queue = BoundedQueue::new(3, Eviction::RefuseNew)?;
    assert_eq!(queue.push(10), Ok(None));
    assert_eq!(queue.push(11), Ok(None));
    assert_eq!(queue.push(12), Ok(None));
    assert_eq!(queue.push(13), Err(13));
    assert_eq!(queue.push(14), Err(14));
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.evictions(), 0, "a refusal is not an eviction");
    assert_eq!(queue.pop(), Some(10));
    assert_eq!(queue.pop(), Some(11));
    assert_eq!(queue.pop(), Some(12));
    assert_eq!(queue.pop(), None);
    Ok(())
}

#[test]
fn queue_refuse_new_accepts_again_once_there_is_room() -> TestResult {
    let mut queue = BoundedQueue::new(2, Eviction::RefuseNew)?;
    assert_eq!(queue.push(1), Ok(None));
    assert_eq!(queue.push(2), Ok(None));
    assert_eq!(queue.push(3), Err(3));
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.push(3), Ok(None));
    assert_eq!(queue.len(), 2);
    Ok(())
}

#[test]
fn queue_evict_oldest_evicts_in_insertion_order() -> TestResult {
    let mut queue = BoundedQueue::new(3, Eviction::EvictOldest)?;
    assert_eq!(queue.push(10), Ok(None));
    assert_eq!(queue.push(11), Ok(None));
    assert_eq!(queue.push(12), Ok(None));
    assert_eq!(queue.evictions(), 0);
    assert_eq!(queue.push(13), Ok(Some(10)));
    assert_eq!(queue.push(14), Ok(Some(11)));
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.evictions(), 2);
    assert_eq!(queue.pop(), Some(12));
    assert_eq!(queue.pop(), Some(13));
    assert_eq!(queue.pop(), Some(14));
    assert_eq!(queue.pop(), None);
    assert!(queue.is_empty());
    assert_eq!(queue.evictions(), 2, "a pop is not an eviction");
    Ok(())
}

#[test]
fn queue_evictions_count_since_construction() -> TestResult {
    let mut queue = BoundedQueue::new(1, Eviction::EvictOldest)?;
    assert_eq!(queue.push(0), Ok(None));
    for (item, evicted, expected_evictions) in
        [(1, 0, 1), (2, 1, 2), (3, 2, 3), (4, 3, 4), (5, 4, 5)]
    {
        assert_eq!(queue.push(item), Ok(Some(evicted)));
        assert_eq!(queue.evictions(), expected_evictions);
    }
    assert_eq!(queue.pop(), Some(5));
    assert_eq!(queue.evictions(), 5);
    Ok(())
}

// List

#[test]
fn list_refuse_new_hands_back_the_item_at_the_cap() -> TestResult {
    let mut list = BoundedList::new(2, Eviction::RefuseNew)?;
    assert_eq!(list.push('a'), Ok(None));
    assert_eq!(list.push('b'), Ok(None));
    assert_eq!(list.push('c'), Err('c'));
    assert_eq!(list.len(), 2);
    assert_eq!(list.evictions(), 0);
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec!['a', 'b']);
    Ok(())
}

#[test]
fn list_evict_oldest_evicts_in_insertion_order() -> TestResult {
    let mut list = BoundedList::new(3, Eviction::EvictOldest)?;
    for item in ['a', 'b', 'c'] {
        assert_eq!(list.push(item), Ok(None));
    }
    assert_eq!(list.push('d'), Ok(Some('a')));
    assert_eq!(list.push('e'), Ok(Some('b')));
    assert_eq!(list.len(), 3);
    assert_eq!(list.evictions(), 2);
    assert_eq!(list.get(0), Some(&'c'));
    assert_eq!(list.get(1), Some(&'d'));
    assert_eq!(list.get(2), Some(&'e'));
    assert_eq!(list.get(3), None);
    assert_eq!(
        list.iter().copied().collect::<Vec<_>>(),
        vec!['c', 'd', 'e']
    );
    Ok(())
}

// Map

#[test]
fn map_refuse_new_hands_back_the_entry_at_the_cap() -> TestResult {
    let mut map = BoundedMap::new(2, Eviction::RefuseNew)?;
    assert_eq!(map.insert("a", 1), Ok(MapInsert::Added));
    assert_eq!(map.insert("b", 2), Ok(MapInsert::Added));
    assert_eq!(map.insert("c", 3), Err(("c", 3)));
    assert_eq!(map.len(), 2);
    assert_eq!(map.evictions(), 0);
    assert!(!map.contains_key(&"c"));
    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    Ok(())
}

#[test]
fn map_evict_oldest_evicts_in_insertion_order() -> TestResult {
    let mut map = BoundedMap::new(2, Eviction::EvictOldest)?;
    assert_eq!(map.insert("a", 1), Ok(MapInsert::Added));
    assert_eq!(map.insert("b", 2), Ok(MapInsert::Added));
    assert_eq!(map.insert("c", 3), Ok(MapInsert::Evicted("a", 1)));
    assert_eq!(map.insert("d", 4), Ok(MapInsert::Evicted("b", 2)));
    assert_eq!(map.len(), 2);
    assert_eq!(map.evictions(), 2);
    assert_eq!(map.get(&"a"), None);
    assert_eq!(map.get(&"b"), None);
    assert_eq!(map.get(&"c"), Some(&3));
    assert_eq!(map.get(&"d"), Some(&4));
    Ok(())
}

#[test]
fn map_insert_of_a_present_key_replaces_without_evicting() -> TestResult {
    for rule in RULES {
        let mut map = BoundedMap::new(2, rule)?;
        assert_eq!(map.insert("a", 1), Ok(MapInsert::Added));
        assert_eq!(map.insert("b", 2), Ok(MapInsert::Added));
        assert_eq!(map.insert("a", 10), Ok(MapInsert::Replaced(1)));
        assert_eq!(map.insert("b", 20), Ok(MapInsert::Replaced(2)));
        assert_eq!(map.len(), 2);
        assert_eq!(map.evictions(), 0);
        assert_eq!(map.get(&"a"), Some(&10));
        assert_eq!(map.get(&"b"), Some(&20));
    }
    Ok(())
}

#[test]
fn map_replacement_keeps_the_key_in_its_insertion_place() -> TestResult {
    let mut map = BoundedMap::new(2, Eviction::EvictOldest)?;
    assert_eq!(map.insert("a", 1), Ok(MapInsert::Added));
    assert_eq!(map.insert("b", 2), Ok(MapInsert::Added));
    assert_eq!(map.insert("a", 10), Ok(MapInsert::Replaced(1)));
    assert_eq!(map.insert("c", 3), Ok(MapInsert::Evicted("a", 10)));
    Ok(())
}

#[test]
fn map_remove_frees_a_slot_without_counting_an_eviction() -> TestResult {
    let mut map = BoundedMap::new(1, Eviction::RefuseNew)?;
    assert_eq!(map.insert("a", 1), Ok(MapInsert::Added));
    assert_eq!(map.remove(&"a"), Some(1));
    assert_eq!(map.remove(&"a"), None);
    assert!(map.is_empty());
    assert_eq!(map.insert("b", 2), Ok(MapInsert::Added));
    assert_eq!(map.evictions(), 0);
    Ok(())
}

// Set

#[test]
fn set_refuse_new_hands_back_the_item_at_the_cap() -> TestResult {
    let mut set = BoundedSet::new(2, Eviction::RefuseNew)?;
    assert_eq!(set.insert(1), Ok(SetInsert::Added));
    assert_eq!(set.insert(2), Ok(SetInsert::Added));
    assert_eq!(set.insert(3), Err(3));
    assert_eq!(set.len(), 2);
    assert_eq!(set.evictions(), 0);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(!set.contains(&3));
    Ok(())
}

#[test]
fn set_evict_oldest_evicts_in_insertion_order() -> TestResult {
    let mut set = BoundedSet::new(2, Eviction::EvictOldest)?;
    assert_eq!(set.insert(1), Ok(SetInsert::Added));
    assert_eq!(set.insert(2), Ok(SetInsert::Added));
    assert_eq!(set.insert(3), Ok(SetInsert::Evicted(1)));
    assert_eq!(set.insert(4), Ok(SetInsert::Evicted(2)));
    assert_eq!(set.len(), 2);
    assert_eq!(set.evictions(), 2);
    assert!(!set.contains(&1));
    assert!(set.contains(&3));
    assert!(set.contains(&4));
    Ok(())
}

#[test]
fn set_duplicate_insert_does_not_evict() -> TestResult {
    for rule in RULES {
        let mut set = BoundedSet::new(2, rule)?;
        assert_eq!(set.insert(1), Ok(SetInsert::Added));
        assert_eq!(set.insert(2), Ok(SetInsert::Added));
        assert_eq!(set.insert(1), Ok(SetInsert::AlreadyPresent));
        assert_eq!(set.insert(2), Ok(SetInsert::AlreadyPresent));
        assert_eq!(set.len(), 2);
        assert_eq!(set.evictions(), 0);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
    }
    Ok(())
}

#[test]
fn set_duplicate_keeps_the_item_in_its_insertion_place() -> TestResult {
    let mut set = BoundedSet::new(2, Eviction::EvictOldest)?;
    assert_eq!(set.insert(1), Ok(SetInsert::Added));
    assert_eq!(set.insert(2), Ok(SetInsert::Added));
    assert_eq!(set.insert(1), Ok(SetInsert::AlreadyPresent));
    assert_eq!(set.insert(3), Ok(SetInsert::Evicted(1)));
    Ok(())
}

#[test]
fn set_remove_frees_a_slot_without_counting_an_eviction() -> TestResult {
    let mut set = BoundedSet::new(1, Eviction::RefuseNew)?;
    assert_eq!(set.insert(1), Ok(SetInsert::Added));
    assert!(set.remove(&1));
    assert!(!set.remove(&1));
    assert!(set.is_empty());
    assert_eq!(set.insert(2), Ok(SetInsert::Added));
    assert_eq!(set.evictions(), 0);
    Ok(())
}

// Byte ring

#[test]
fn byte_ring_holds_writes_under_the_cap() -> TestResult {
    let mut ring = ByteRing::new(8, Eviction::EvictOldest)?;
    assert_eq!(ring.push(b"abc"), Ok(0));
    assert_eq!(ring.push(b"de"), Ok(0));
    assert_eq!(ring.to_vec(), b"abcde".to_vec());
    assert_eq!(ring.len(), 5);
    assert_eq!(ring.evictions(), 0);
    Ok(())
}

#[test]
fn byte_ring_keeps_the_newest_cap_bytes_after_an_overflow() -> TestResult {
    let mut ring = ByteRing::new(8, Eviction::EvictOldest)?;
    assert_eq!(ring.push(b"abcdef"), Ok(0));
    assert_eq!(ring.push(b"ghijk"), Ok(3));
    assert_eq!(ring.to_vec(), b"defghijk".to_vec());
    assert_eq!(ring.len(), 8);
    assert_eq!(ring.evictions(), 3);
    assert_eq!(ring.push(b"lm"), Ok(2));
    assert_eq!(ring.to_vec(), b"fghijklm".to_vec());
    assert_eq!(ring.evictions(), 5);
    Ok(())
}

#[test]
fn byte_ring_write_longer_than_the_cap_keeps_its_tail() -> TestResult {
    let mut ring = ByteRing::new(4, Eviction::EvictOldest)?;
    assert_eq!(ring.push(b"xy"), Ok(0));
    assert_eq!(ring.push(b"0123456789"), Ok(8));
    assert_eq!(ring.to_vec(), b"6789".to_vec());
    assert_eq!(ring.len(), 4);
    assert_eq!(ring.evictions(), 8);
    Ok(())
}

#[test]
fn byte_ring_empty_write_changes_nothing() -> TestResult {
    let mut ring = ByteRing::new(2, Eviction::EvictOldest)?;
    assert_eq!(ring.push(b"ab"), Ok(0));
    assert_eq!(ring.push(b""), Ok(0));
    assert_eq!(ring.to_vec(), b"ab".to_vec());
    assert_eq!(ring.evictions(), 0);
    Ok(())
}

// Growth

#[test]
fn a_large_cap_does_not_preallocate() -> TestResult {
    for rule in RULES {
        let mut queue = BoundedQueue::new(LARGE_CAP, rule)?;
        assert!(
            queue.allocated() <= SMALL_ALLOCATION,
            "queue: {}",
            queue.allocated()
        );
        let mut map = BoundedMap::new(LARGE_CAP, rule)?;
        assert!(
            map.allocated() <= SMALL_ALLOCATION,
            "map: {}",
            map.allocated()
        );
        let mut set = BoundedSet::new(LARGE_CAP, rule)?;
        assert!(
            set.allocated() <= SMALL_ALLOCATION,
            "set: {}",
            set.allocated()
        );
        let mut list = BoundedList::new(LARGE_CAP, rule)?;
        assert!(
            list.allocated() <= SMALL_ALLOCATION,
            "list: {}",
            list.allocated()
        );

        for item in 0_u8..3 {
            assert_eq!(queue.push(item), Ok(None));
            assert_eq!(map.insert(item, item), Ok(MapInsert::Added));
            assert_eq!(set.insert(item), Ok(SetInsert::Added));
            assert_eq!(list.push(item), Ok(None));
        }
        assert!(
            queue.allocated() <= SMALL_ALLOCATION,
            "queue: {}",
            queue.allocated()
        );
        assert!(
            map.allocated() <= SMALL_ALLOCATION,
            "map: {}",
            map.allocated()
        );
        assert!(
            set.allocated() <= SMALL_ALLOCATION,
            "set: {}",
            set.allocated()
        );
        assert!(
            list.allocated() <= SMALL_ALLOCATION,
            "list: {}",
            list.allocated()
        );
    }

    let mut ring = ByteRing::new(LARGE_CAP, Eviction::EvictOldest)?;
    assert!(
        ring.allocated() <= SMALL_ALLOCATION,
        "ring: {}",
        ring.allocated()
    );
    assert_eq!(ring.push(b"abc"), Ok(0));
    assert!(
        ring.allocated() <= SMALL_ALLOCATION,
        "ring: {}",
        ring.allocated()
    );
    Ok(())
}

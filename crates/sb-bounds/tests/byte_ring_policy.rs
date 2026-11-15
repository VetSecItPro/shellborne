//! The byte ring's eviction rule (plan step P0.10.7): its constructor takes a cap and a rule like
//! every other bounded collection, a full ring under [`Eviction::RefuseNew`] keeps what it holds and
//! reports every byte it did not store, and [`Eviction::EvictOldest`] keeps the newest bytes.

use sb_bounds::{ByteRing, Eviction, RingRefused, ZeroCap};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const RULES: [Eviction; 2] = [Eviction::RefuseNew, Eviction::EvictOldest];

#[test]
fn zero_cap_is_refused_under_every_rule() {
    for rule in RULES {
        assert_eq!(ByteRing::new(0, rule).err(), Some(ZeroCap));
    }
}

#[test]
fn cap_of_one_is_accepted_under_every_rule() -> TestResult {
    for rule in RULES {
        let ring = ByteRing::new(1, rule)?;
        assert_eq!((ring.cap(), ring.len(), ring.is_empty()), (1, 0, true));
        assert_eq!(ring.evictions(), 0);
    }
    Ok(())
}

#[test]
fn refuse_new_stores_writes_that_fit() -> TestResult {
    let mut ring = ByteRing::new(5, Eviction::RefuseNew)?;
    assert_eq!(ring.push(b"abc"), Ok(0));
    assert_eq!(ring.push(b"de"), Ok(0));
    assert_eq!(ring.to_vec(), b"abcde".to_vec());
    assert_eq!(ring.evictions(), 0);
    Ok(())
}

#[test]
fn refuse_new_keeps_the_held_prefix_and_reports_the_rejected_bytes() -> TestResult {
    let mut ring = ByteRing::new(8, Eviction::RefuseNew)?;
    assert_eq!(ring.push(b"abcdef"), Ok(0));
    assert_eq!(
        ring.push(b"ghijk"),
        Err(RingRefused {
            accepted: 2,
            rejected: 3
        })
    );
    assert_eq!(ring.to_vec(), b"abcdefgh".to_vec());
    assert_eq!(ring.len(), 8);
    assert_eq!(ring.evictions(), 0, "a refusal is not an eviction");
    Ok(())
}

#[test]
fn refuse_new_when_full_rejects_the_whole_write() -> TestResult {
    let mut ring = ByteRing::new(2, Eviction::RefuseNew)?;
    assert_eq!(ring.push(b"ab"), Ok(0));
    assert_eq!(
        ring.push(b"cd"),
        Err(RingRefused {
            accepted: 0,
            rejected: 2
        })
    );
    assert_eq!(
        ring.push(b"e"),
        Err(RingRefused {
            accepted: 0,
            rejected: 1
        })
    );
    assert_eq!(ring.to_vec(), b"ab".to_vec());
    assert_eq!(ring.evictions(), 0);
    Ok(())
}

#[test]
fn refuse_new_write_longer_than_the_cap_keeps_its_head() -> TestResult {
    let mut ring = ByteRing::new(4, Eviction::RefuseNew)?;
    assert_eq!(
        ring.push(b"0123456789"),
        Err(RingRefused {
            accepted: 4,
            rejected: 6
        })
    );
    assert_eq!(ring.to_vec(), b"0123".to_vec());
    assert_eq!(ring.evictions(), 0);
    Ok(())
}

#[test]
fn refuse_new_empty_write_on_a_full_ring_is_not_a_refusal() -> TestResult {
    let mut ring = ByteRing::new(2, Eviction::RefuseNew)?;
    assert_eq!(ring.push(b"ab"), Ok(0));
    assert_eq!(ring.push(b""), Ok(0));
    assert_eq!(ring.to_vec(), b"ab".to_vec());
    Ok(())
}

#[test]
fn refusal_is_a_displayable_error() {
    let refused = RingRefused {
        accepted: 1,
        rejected: 2,
    };
    let as_error: &dyn std::error::Error = &refused;
    assert!(!as_error.to_string().is_empty());
}

#[test]
fn evict_oldest_keeps_the_newest_cap_bytes() -> TestResult {
    let mut ring = ByteRing::new(8, Eviction::EvictOldest)?;
    assert_eq!(ring.push(b"abcdef"), Ok(0));
    assert_eq!(ring.push(b"ghijk"), Ok(3));
    assert_eq!(ring.to_vec(), b"defghijk".to_vec());
    assert_eq!(ring.evictions(), 3);
    assert_eq!(ring.push(b"lm"), Ok(2));
    assert_eq!(ring.to_vec(), b"fghijklm".to_vec());
    assert_eq!(ring.evictions(), 5);
    Ok(())
}

#[test]
fn evict_oldest_write_longer_than_the_cap_keeps_its_tail() -> TestResult {
    let mut ring = ByteRing::new(4, Eviction::EvictOldest)?;
    assert_eq!(ring.push(b"xy"), Ok(0));
    assert_eq!(ring.push(b"0123456789"), Ok(8));
    assert_eq!(ring.to_vec(), b"6789".to_vec());
    assert_eq!(ring.evictions(), 8);
    Ok(())
}

use super::*;

#[test]
fn deadlines_reject_both_duration_and_addition_overflow() {
    use std::time::Duration;
    assert_eq!(deadline(0, Duration::ZERO), Ok(0));
    assert_eq!(deadline(u64::MAX, Duration::ZERO), Ok(u64::MAX));
    assert_eq!(
        deadline(u64::MAX, Duration::from_nanos(1)),
        Err(AdmissionError::DeadlineOverflow)
    );
    assert_eq!(
        deadline(0, Duration::from_secs(u64::MAX)),
        Err(AdmissionError::DeadlineOverflow)
    );
}

#[test]
fn admission_is_atomic_and_identities_never_wrap_or_recur() {
    let mut queue = Queue::<_, 1>::new();
    let first = queue.insert(10, "first").unwrap();
    assert_eq!(
        queue.insert(1, "rejected"),
        Err((AdmissionError::Capacity, "rejected"))
    );
    assert_eq!(queue.next(), Some((10, first)));
    assert_eq!(queue.remove(first), Some("first"));
    let second = queue.insert(1, "second").unwrap();
    assert_ne!(first, second);
    assert_eq!(queue.remove(first), None);
    assert_eq!(queue.remove(second), Some("second"));
    queue.last_id = u64::MAX;
    assert_eq!(
        queue.insert(0, "overflow"),
        Err((AdmissionError::IdentityExhausted, "overflow"))
    );
    assert_eq!(queue.next(), None);
}

#[test]
fn ordering_submission_retry_and_late_acceptance_share_one_task() {
    let mut queue = Queue::<_, 3>::new();
    let first = queue.insert(5, 1).unwrap();
    let second = queue.insert(5, 2).unwrap();
    let third = queue.insert(4, 3).unwrap();
    queue.submit(third);
    assert_eq!(queue.next(), Some((5, first)));
    queue.submit(first);
    assert_eq!(queue.next(), Some((5, second)));
    queue.retry(third, 6);
    assert_eq!(queue.remove(third), Some(3)); // late request wins once
    queue.retry(third, 7); // its transport response cannot resurrect it
    assert_eq!(queue.remove(third), None);
    assert_eq!(queue.remove(first), Some(1));
    assert_eq!(queue.remove(second), Some(2));
    assert_eq!(queue.next(), None);
}

#[test]
fn cancellation_churn_retains_no_deadline_records() {
    let mut queue = Queue::<_, 2>::new();
    let survivor = queue.insert(20, ()).unwrap();
    for _ in 0..10_000 {
        let id = queue.insert(u64::MAX, ()).unwrap();
        assert_eq!(queue.remove(id), Some(()));
        assert_eq!(queue.entries.len(), 1);
        assert_eq!(queue.next(), Some((20, survivor)));
    }
}

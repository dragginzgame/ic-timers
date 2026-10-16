//! Real self-call queue pressure without changing the timer backend.

use std::cell::Cell;

thread_local! {
    static SUBMISSION_FAILURES: Cell<u64> = const { Cell::new(0) };
    static REPLIES: Cell<u64> = const { Cell::new(0) };
    static ENABLED: Cell<bool> = const { Cell::new(true) };
}

#[ic_cdk::update]
fn pressure_and_start() {
    for _ in 0..600 {
        send_pressure();
    }
    assert!(super::start_immediately());
}

fn send_pressure() {
    ic_cdk::futures::spawn(async {
        match ic_cdk::call::Call::unbounded_wait(ic_cdk::api::canister_self(), "pressure_reply")
            .await
        {
            Ok(_) => {
                REPLIES.set(REPLIES.get() + 1);
                if ENABLED.get() {
                    send_pressure();
                }
            }
            Err(ic_cdk::call::CallFailed::CallPerformFailed(_)) => {
                SUBMISSION_FAILURES.set(SUBMISSION_FAILURES.get() + 1);
            }
            Err(error) => ic_cdk::trap(format!("unexpected pressure error: {error}")),
        }
    });
}

#[ic_cdk::update]
fn release_pressure() {
    ENABLED.set(false);
}

#[ic_cdk::update]
fn pressure_reply() {}

#[ic_cdk::query]
fn pressure_observation() -> (u64, u64) {
    (SUBMISSION_FAILURES.get(), REPLIES.get())
}

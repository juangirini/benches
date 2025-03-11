use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

#[derive(PartialEq, Copy, Clone)]
enum State {
    Active,
    Paused,
}
#[derive(Copy, Clone)]
struct Subscription {
    state: State,
    last_delivered: Option<u32>,
}
#[derive(Copy, Clone)]
struct SubscriptionNoOption {
    state: State,
    last_delivered: u32,
}

/// Builds a vector of 10M subscriptions with 10% of them paused
fn build_some_paused() -> Vec<Subscription> {
    let active = 9_000_000;
    let paused = 1_000_000;
    let mut subscriptions: Vec<Subscription> = vec![];
    for _ in 0..active {
        subscriptions.push(Subscription {
            state: State::Active,
            last_delivered: Some(1),
        });
    }
    for _ in 0..paused {
        subscriptions.push(Subscription {
            state: State::Paused,
            last_delivered: Some(1),
        });
    }
    return subscriptions;
}

/// Builds a vector of 9M subscriptions all active
fn build_only_active() -> Vec<Subscription> {
    let active = 9_000_000;
    let mut subscriptions: Vec<Subscription> = vec![];
    for _ in 0..active {
        subscriptions.push(Subscription {
            state: State::Active,
            last_delivered: Some(1),
        });
    }
    return subscriptions;
}

fn iter_sub(subscriptions: Vec<Subscription>) {
    for _sub in subscriptions
        .iter()
        .filter(|sub| sub.state == State::Active && sub.last_delivered.unwrap_or(0u32) > 1)
    {}
}

/// Builds a vector of 10M subscriptions with 10% of them paused
fn build_some_paused_no() -> Vec<SubscriptionNoOption> {
    let active = 9_000_000;
    let paused = 1_000_000;
    let mut subscriptions: Vec<SubscriptionNoOption> = vec![];
    for _ in 0..active {
        subscriptions.push(SubscriptionNoOption {
            state: State::Active,
            last_delivered: 0,
        });
    }
    for _ in 0..paused {
        subscriptions.push(SubscriptionNoOption {
            state: State::Paused,
            last_delivered: 0
        });
    }
    return subscriptions;
}

/// Builds a vector of 9M subscriptions all active
fn build_only_active_no() -> Vec<SubscriptionNoOption> {
    let active = 9_000_000;
    let mut subscriptions: Vec<SubscriptionNoOption> = vec![];
    for _ in 0..active {
        subscriptions.push(SubscriptionNoOption {
            state: State::Active,
            last_delivered: 0,
        });
    }
    return subscriptions;
}

fn iter_sub_no(subscriptions: Vec<SubscriptionNoOption>) {
    for _sub in subscriptions
        .iter()
        .filter(|sub| sub.state == State::Active && sub.last_delivered > 1)
    {}
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // last_delivered being an option
    let subs = build_some_paused();
    c.bench_function("sub 9M active + 1M paused", |b| {
        b.iter(|| iter_sub(subs.clone()))
    });
    let subs = build_only_active();
    c.bench_function("sub 9M active", |b| b.iter(|| iter_sub(subs.clone())));

    // laste_delivered not being an option
    let subs = build_some_paused_no();
    c.bench_function("sub noopt 9M active + 1M paused", |b| {
        b.iter(|| iter_sub_no(subs.clone()))
    });
    let subs = build_only_active_no();
    c.bench_function("sub noopt 9M active", |b| b.iter(|| iter_sub_no(subs.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

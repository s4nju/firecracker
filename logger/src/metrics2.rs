use std::sync::atomic::{AtomicUsize, Ordering};

use chrono;
use serde::{Serialize, Serializer};

// The main design goals of this metrics system are:
// - Use lockless operations, preferably ones that don't require anything other than
// simple reads/writes being atomic.
// - Exploit interior mutability and atomics being Sync to allow all methods (including the ones
// which are effectively mutable) to be callable on a global non-mut static.
// - Rely on Serde to provide the actual serialization for logging the metrics.
// - Since all metrics start at 0, we implement the Default trait via derive for all of them,
// to avoid having to initialize everything by hand.

// Moreover, the value of a metric is currently NOT reset to 0 each time it's being logged. There
// are a number of advantages to this approach, including:
// - We don't have to introduce an additional write (to reset the value) from the thread which
// does to actual logging, so less synchronization effort is required.
// - We don't have to worry at all that much about losing some data if logging fails for a while
// (this could be a concern, I guess).
//
// If if turns out this approach is not really what we want, it's pretty easy to resort to something
// else, while working behind the same interface.

// This trait helps with writing less code. It has to be in scope (via an use directive) in order
// for its methods to be available to call on structs that implement it.
pub trait Metric {
    fn add(&self, value: usize);
    fn inc(&self) {
        self.add(1);
    }
    fn count(&self) -> usize;
}

// A simple metric is one meant to be incremented from a single thread, so it can use simple
// loads + stores. Loads are currently Relaxed everywhere, because we don't do anything besides
// logging the retrieved value (their outcome os not used to modify some memory location in a
// potentially inconsistent manner). There's no way currently to make sure a SimpleMetric is only
// incremented by a single thread, this has to be enforced via judicious use (although, every
// non-vCPU related metric is associated with a particular thread, so it shouldn't be that easy
// to misuse SimpleMetric fields).
#[derive(Default)]
pub struct SimpleMetric(AtomicUsize);

impl Metric for SimpleMetric {
    fn add(&self, value: usize) {
        let ref count = self.0;
        count.store(count.load(Ordering::Relaxed) + value, Ordering::Relaxed);
    }

    fn count(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

impl Serialize for SimpleMetric {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // There's no serializer.serialize_usize() for some reason :(
        serializer.serialize_u64(self.0.load(Ordering::Relaxed) as u64)
    }
}

// A shared metric is one which is expected to be incremented from more than one thread, so more
// synchronization is necessary. It's currently used for vCPU metrics. An alternative here would be
// to have one instance of every metric for each thread (like a per-thread SimpleMetric), and to
// aggregate them when logging. However this probably overkill unless we have a lot of vCPUs
// incrementing metrics very often. Still, it's there if we ever need it :-s
#[derive(Default)]
pub struct SharedMetric(AtomicUsize);

impl Metric for SharedMetric {
    // While the order specified for this operation is still Relaxed, the actual instruction will
    // be an asm "LOCK; something" and thus atomic across multiple threads, simply because of the
    // fetch_and_add (as opposed to "store(load() + 1)") implementation for atomics.
    // TODO: would a stronger ordering make a difference here?
    fn add(&self, value: usize) {
        self.0.fetch_add(value, Ordering::Relaxed);
    }

    fn count(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

impl Serialize for SharedMetric {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // There's no serializer.serialize_usize() for some reason :(
        serializer.serialize_u64(self.0.load(Ordering::Relaxed) as u64)
    }
}

// The following structs are used to define a certain organization for the set of metrics we
// are interested in. Whenever the name of a field differs from its ideal textual representation
// in the serialized form, we can use the #[serde(rename = "name")] attribute to, well, rename it.

#[derive(Default, Serialize)]
pub struct ApiMetrics {}

#[derive(Default, Serialize)]
pub struct BlockDeviceMetrics {
    pub activate_fails: SharedMetric,
    pub cfg_fails: SharedMetric,
    pub event_fails: SharedMetric,
    pub execute_fails: SharedMetric,
    pub invalid_reqs_count: SharedMetric,
    pub flush_count: SharedMetric,
    pub queue_event_count: SharedMetric,
    pub rate_limiter_event_count: SharedMetric,
    pub read_count: SharedMetric,
    pub write_count: SharedMetric,
}

#[derive(Default, Serialize)]
pub struct I8042DeviceMetrics {
    pub error_count: SharedMetric,
    pub missed_read_count: SharedMetric,
    pub missed_write_count: SharedMetric,
    pub read_count: SharedMetric,
    pub reset_count: SharedMetric,
    pub write_count: SharedMetric,
}

#[derive(Default, Serialize)]
pub struct NetDeviceMetrics {
    pub activate_fails: SharedMetric,
    pub cfg_fails: SharedMetric,
    pub event_fails: SharedMetric,
    pub rx_bytes_count: SharedMetric,
    pub rx_packets_count: SharedMetric,
    pub rx_event_rate_limiter_count: SharedMetric,
    pub rx_fails: SharedMetric,
    pub rx_queue_event_count: SharedMetric,
    pub rx_tap_event_count: SharedMetric,
    pub tx_bytes_count: SharedMetric,
    pub tx_packets_count: SharedMetric,
    pub tx_rate_limiter_event_count: SharedMetric,
    pub tx_fails: SharedMetric,
    pub tx_queue_event_count: SharedMetric,
}

#[derive(Default, Serialize)]
pub struct SerialDeviceMetrics {
    pub error_count: SharedMetric,
    pub flush_count: SharedMetric,
    pub missed_read_count: SharedMetric,
    pub missed_write_count: SharedMetric,
    pub read_count: SharedMetric,
    pub write_count: SharedMetric,
}

#[derive(Default, Serialize)]
pub struct VcpuMetrics {
    pub eagain: SharedMetric,
    pub eintr: SharedMetric,
    pub exit_io_in: SharedMetric,
    pub exit_io_out: SharedMetric,
    pub exit_mmio_read: SharedMetric,
    pub exit_mmio_write: SharedMetric,
    pub failures: SharedMetric,
}

#[derive(Default, Serialize)]
pub struct VmmMetrics {
    pub device_events: SharedMetric,
}

// The sole purpose of this struct is to produce an UTC timestamp when an instance is serialized.
#[derive(Default)]
struct SerializeToUtcTimestampMs;

impl Serialize for SerializeToUtcTimestampMs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(chrono::Utc::now().timestamp_millis())
    }
}

#[derive(Default, Serialize)]
pub struct FirecrackerMetrics {
    utc_timestamp_ms: SerializeToUtcTimestampMs,
    pub api: ApiMetrics,
    pub block: BlockDeviceMetrics,
    pub i8042: I8042DeviceMetrics,
    pub net: NetDeviceMetrics,
    pub vcpu: VcpuMetrics,
    pub vmm: VmmMetrics,
    pub uart: SerialDeviceMetrics,
}

// The global variable used to increase the value of various metrics.
lazy_static! {
    pub static ref METRICS: FirecrackerMetrics = FirecrackerMetrics::default();
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_metric() {
        let m1 = SimpleMetric::default();

        m1.inc();
        m1.inc();
        m1.add(5);
        m1.inc();

        assert_eq!(m1.count(), 8);

        let m2 = Arc::new(SharedMetric::default());

        // We're going to create a number of threads that will attempt to increase this metric
        // in parallel. If everything goes fine we still can't be sure the synchronization works,
        // but it something fails, then we definitely have a problem :-s

        const NUM_THREADS_TO_SPAWN: usize = 4;
        const NUM_INCREMENTS_PER_THREAD: usize = 100000;
        const M2_INITIAL_COUNT: usize = 123;

        m2.add(M2_INITIAL_COUNT);

        let mut v = Vec::with_capacity(NUM_THREADS_TO_SPAWN);

        for _ in 0..NUM_THREADS_TO_SPAWN {
            let r = m2.clone();
            v.push(thread::spawn(move || {
                for _ in 0..NUM_INCREMENTS_PER_THREAD {
                    r.inc();
                }
            }));
        }

        for handle in v {
            handle.join().unwrap();
        }

        assert_eq!(
            m2.count(),
            M2_INITIAL_COUNT + NUM_THREADS_TO_SPAWN * NUM_INCREMENTS_PER_THREAD
        );
    }
}
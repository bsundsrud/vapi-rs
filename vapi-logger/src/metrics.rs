use lazy_static::lazy_static;
use prometheus::{Encoder, Histogram, HistogramOpts, IntCounter, Registry};

lazy_static! {
    pub static ref SENT_COUNTER: IntCounter =
        IntCounter::new("send_count", "logs sent to output").unwrap();
    pub static ref OVERRUN_COUNTER: IntCounter =
        IntCounter::new("overrun_count", "count of log overruns").unwrap();
    pub static ref SENT_HISTO: Histogram = Histogram::with_opts(HistogramOpts::new(
        "send_duration_seconds",
        "time to send logs to output, in seconds"
    ))
    .unwrap();
    pub static ref RECONNECT_COUNTER: IntCounter = IntCounter::new(
        "reconnect_count",
        "count of reconnections to output destination"
    )
    .unwrap();
}
#[derive(Debug, Clone)]
pub struct Metrics {
    registry: Registry,
}

impl Metrics {
    pub fn new(prefix: &str) -> Metrics {
        let registry = Registry::new_custom(Some(prefix.to_string()), None).unwrap();
        registry.register(Box::new(SENT_COUNTER.clone())).unwrap();
        registry
            .register(Box::new(OVERRUN_COUNTER.clone()))
            .unwrap();
        registry.register(Box::new(SENT_HISTO.clone())).unwrap();
        registry
            .register(Box::new(RECONNECT_COUNTER.clone()))
            .unwrap();
        Metrics { registry }
    }

    pub fn print_metrics(&self) {
        println!("{}", self.get_metrics_text());
    }

    pub fn get_metrics_text(&self) -> String {
        let mut buffer = Vec::<u8>::new();
        let encoder = prometheus::TextEncoder::new();
        encoder
            .encode(&self.registry.gather(), &mut buffer)
            .unwrap();

        String::from_utf8(buffer).unwrap()
    }
}

use prometheus::{IntCounter, HistogramOpts, HistogramVec, Opts, Registry};

lazy_static::lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    pub static ref REQUEST_COUNTER: IntCounter = IntCounter::with_opts(
        Opts::new("http_requests_total", "Total HTTP requests")
    ).unwrap();

    pub static ref REQUEST_DURATION_HISTOGRAM: HistogramVec = HistogramVec::new(
        HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
            .buckets(vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]),
        &["method", "path"]
    ).unwrap();
}

pub fn init() {
    REGISTRY.register(Box::new(REQUEST_COUNTER.clone())).unwrap();
    REGISTRY
        .register(Box::new(REQUEST_DURATION_HISTOGRAM.clone()))
        .unwrap();
}

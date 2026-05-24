use prometheus::{default_registry, HistogramOpts, HistogramVec, IntCounter, Opts};

#[allow(clippy::expect_used)]
fn request_counter() -> &'static IntCounter {
    static COUNTER: std::sync::OnceLock<IntCounter> = std::sync::OnceLock::new();
    COUNTER.get_or_init(|| {
        IntCounter::with_opts(Opts::new("http_requests_total", "Total HTTP requests"))
            .expect("Hardcoded metric name is always valid")
    })
}

#[allow(clippy::expect_used)]
fn request_duration_histogram() -> &'static HistogramVec {
    static HISTOGRAM: std::sync::OnceLock<HistogramVec> = std::sync::OnceLock::new();
    HISTOGRAM.get_or_init(|| {
        HistogramVec::new(
            HistogramOpts::new("http_request_duration_seconds", "HTTP request duration").buckets(
                vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0],
            ),
            &["method", "path"],
        )
        .expect("Hardcoded metric config is always valid")
    })
}

pub fn init() {
    let registry = default_registry();
    if let Err(e) = registry.register(Box::new(request_counter().clone())) {
        tracing::warn!(error = %e, "request_counter already registered");
    }
    if let Err(e) = registry.register(Box::new(request_duration_histogram().clone())) {
        tracing::warn!(error = %e, "request_duration_histogram already registered");
    }
}

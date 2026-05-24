use tracing_subscriber::fmt;

pub fn init_tracing() {
    fmt()
        .json()
        .with_target(false)
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .init();
}

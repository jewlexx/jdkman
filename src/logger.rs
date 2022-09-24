use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

const LOG_LEVEL: Level = {
    if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    }
};

#[instrument]
pub fn init_subscriber() {
    debug!("Initializing logger");

    let sub = tracing_subscriber::fmt().with_max_level(LOG_LEVEL);

    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            sub.with_span_events(FmtSpan::NEW | FmtSpan::CLOSE).init();
        } else {
            sub.init();
        }
    }

    debug!("Logger initialized");
}

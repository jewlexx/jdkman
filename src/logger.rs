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
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            let events = FmtSpan::NEW | FmtSpan::CLOSE | FmtSpan::ENTER | FmtSpan::EXIT;
        } else {
            let events = FmtSpan::CLOSE;
        }
    }

    debug!("Initializing logger");

    tracing_subscriber::fmt()
        .with_max_level(LOG_LEVEL)
        .with_span_events(events)
        .with_target(false)
        .init();

    debug!("Logger initialized");
}

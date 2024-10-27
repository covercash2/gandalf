use std::str::FromStr as _;

use http::HeaderName;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter,
};

pub mod api_key;
pub mod error;
pub mod io;

pub const CONFIG_DIR_PREFIX: &str = "gandalf";
pub const KEY_HEADER: HeaderName = HeaderName::from_static("fellowship");

pub fn setup_tracing(level: &str) {
    let log_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .json();

    tracing_subscriber::Registry::default()
        .with(EnvFilter::from_str(level).unwrap())
        .with(log_layer)
        .init();
}

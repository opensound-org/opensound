use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TraceLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TraceData {
    level: TraceLevel,
    message: String,
    module: String,
    source: Option<String>,
    payload: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventData {
    module: String,
    source: String,
    payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Tracing {
    Trace(TraceData),
    Event(EventData),
}

pub mod emit {
    use super::TraceLevel;
    use serde::Serialize;

    pub fn trace(message: &str, module: &str, source: Option<&str>, timestamp: bool) {
        emit::<()>(TraceLevel::Trace, message, module, source, None, timestamp);
    }

    pub fn trace_payload(
        message: &str,
        module: &str,
        source: Option<&str>,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Trace,
            message,
            module,
            source,
            Some(payload),
            timestamp,
        );
    }

    pub fn debug(message: &str, module: &str, source: Option<&str>, timestamp: bool) {
        emit::<()>(TraceLevel::Debug, message, module, source, None, timestamp);
    }

    pub fn debug_payload(
        message: &str,
        module: &str,
        source: Option<&str>,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Debug,
            message,
            module,
            source,
            Some(payload),
            timestamp,
        );
    }

    pub fn info(message: &str, module: &str, source: Option<&str>, timestamp: bool) {
        emit::<()>(TraceLevel::Info, message, module, source, None, timestamp);
    }

    pub fn info_payload(
        message: &str,
        module: &str,
        source: Option<&str>,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Info,
            message,
            module,
            source,
            Some(payload),
            timestamp,
        );
    }

    pub fn warn(message: &str, module: &str, source: Option<&str>, timestamp: bool) {
        emit::<()>(TraceLevel::Warn, message, module, source, None, timestamp);
    }

    pub fn warn_payload(
        message: &str,
        module: &str,
        source: Option<&str>,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Warn,
            message,
            module,
            source,
            Some(payload),
            timestamp,
        );
    }

    pub fn error(message: &str, module: &str, source: Option<&str>, timestamp: bool) {
        emit::<()>(TraceLevel::Error, message, module, source, None, timestamp);
    }

    pub fn error_payload(
        message: &str,
        module: &str,
        source: Option<&str>,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Error,
            message,
            module,
            source,
            Some(payload),
            timestamp,
        );
    }

    fn emit<T: Serialize>(
        level: TraceLevel,
        message: &str,
        module: &str,
        source: Option<&str>,
        payload: Option<T>,
        timestamp: bool,
    ) {
        let payload = || payload.map(|p| serde_json::to_string(&p).unwrap());
        let utc = || {
            if timestamp {
                Some(format!("{:?}", chrono::Utc::now().naive_utc()))
            } else {
                None
            }
        };

        match level {
            TraceLevel::Trace => {
                tracing::trace!(target: "ostp", message, module, source, utc = utc(), payload = payload())
            }
            TraceLevel::Debug => {
                tracing::debug!(target: "ostp", message, module, source, utc = utc(), payload = payload())
            }
            TraceLevel::Info => {
                tracing::info!(target: "ostp", message, module, source, utc = utc(), payload = payload())
            }
            TraceLevel::Warn => {
                tracing::warn!(target: "ostp", message, module, source, utc = utc(), payload = payload())
            }
            TraceLevel::Error => {
                tracing::error!(target: "ostp", message, module, source, utc = utc(), payload = payload())
            }
        }
    }
}

use tracing_subscriber::fmt::{format::FmtSpan, time::ChronoLocal};

#[cfg(debug_assertions)]
pub fn install_default() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_timer(ChronoLocal::new("%m-%d %H:%M:%S".into()))
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .with_thread_names(true)
        .init();
}

#[cfg(not(debug_assertions))]
pub fn install_default() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_timer(ChronoLocal::new("%m-%d %H:%M:%S".into()))
        .with_span_events(FmtSpan::FULL)
        .with_thread_names(true)
        .init();
}

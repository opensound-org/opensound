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
    module: String,
    source: Option<String>,
    message: String,
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

    pub fn trace(module: &str, source: Option<&str>, message: &str, timestamp: bool) {
        emit::<()>(TraceLevel::Trace, module, source, message, None, timestamp);
    }

    pub fn trace_payload(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Trace,
            module,
            source,
            message,
            Some(payload),
            timestamp,
        );
    }

    pub fn debug(module: &str, source: Option<&str>, message: &str, timestamp: bool) {
        emit::<()>(TraceLevel::Debug, module, source, message, None, timestamp);
    }

    pub fn debug_payload(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Debug,
            module,
            source,
            message,
            Some(payload),
            timestamp,
        );
    }

    pub fn info(module: &str, source: Option<&str>, message: &str, timestamp: bool) {
        emit::<()>(TraceLevel::Info, module, source, message, None, timestamp);
    }

    pub fn info_payload(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Info,
            module,
            source,
            message,
            Some(payload),
            timestamp,
        );
    }

    pub fn warn(module: &str, source: Option<&str>, message: &str, timestamp: bool) {
        emit::<()>(TraceLevel::Warn, module, source, message, None, timestamp);
    }

    pub fn warn_payload(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Warn,
            module,
            source,
            message,
            Some(payload),
            timestamp,
        );
    }

    pub fn error(module: &str, source: Option<&str>, message: &str, timestamp: bool) {
        emit::<()>(TraceLevel::Error, module, source, message, None, timestamp);
    }

    pub fn error_payload(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: impl Serialize,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Error,
            module,
            source,
            message,
            Some(payload),
            timestamp,
        );
    }

    fn emit<T: Serialize>(
        level: TraceLevel,
        module: &str,
        source: Option<&str>,
        message: &str,
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

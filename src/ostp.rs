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

    pub fn trace<T: Serialize>(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: Option<T>,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Trace,
            module,
            source,
            message,
            payload,
            timestamp,
        );
    }

    pub fn debug<T: Serialize>(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: Option<T>,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Debug,
            module,
            source,
            message,
            payload,
            timestamp,
        );
    }

    pub fn info<T: Serialize>(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: Option<T>,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Info,
            module,
            source,
            message,
            payload,
            timestamp,
        );
    }

    pub fn warn<T: Serialize>(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: Option<T>,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Warn,
            module,
            source,
            message,
            payload,
            timestamp,
        );
    }

    pub fn error<T: Serialize>(
        module: &str,
        source: Option<&str>,
        message: &str,
        payload: Option<T>,
        timestamp: bool,
    ) {
        emit(
            TraceLevel::Error,
            module,
            source,
            message,
            payload,
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
        let payload = payload.map(|p| serde_json::to_string(&p).unwrap());
        let utc = if timestamp {
            Some(serde_json::to_string(&chrono::Utc::now().naive_utc()).unwrap())
        } else {
            None
        };

        match level {
            TraceLevel::Trace => {
                tracing::trace!(target: "ostp", utc, module, source, message, payload)
            }
            TraceLevel::Debug => {
                tracing::debug!(target: "ostp", utc, module, source, message, payload)
            }
            TraceLevel::Info => {
                tracing::info!(target: "ostp", utc, module, source, message, payload)
            }
            TraceLevel::Warn => {
                tracing::warn!(target: "ostp", utc, module, source, message, payload)
            }
            TraceLevel::Error => {
                tracing::error!(target: "ostp", utc, module, source, message, payload)
            }
        }
    }
}

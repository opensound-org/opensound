use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use sys_locale::get_locale;

#[derive(Debug, Deserialize, Serialize)]
pub enum TraceLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceInfo {
    gadget: String,
    unit: String,
    source_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TraceData {
    level: TraceLevel,
    message_en: String,
    message_zh: Option<String>,
    source_info: SourceInfo,
    payload: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventData {
    gadget: String,
    unit: String,
    event_type: String,
    payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Tracing {
    Trace(TraceData),
    Event(EventData),
}

const IS_ZH_CN: OnceLock<bool> = OnceLock::new();

pub fn is_zh_cn() -> bool {
    *IS_ZH_CN.get_or_init(|| get_locale() == Some("zh-CN".into()))
}

pub mod emit {
    use super::TraceLevel;
    use serde::Serialize;

    pub fn trace(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
    ) {
        emit::<()>(
            TraceLevel::Trace,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            None,
            true,
        );
    }

    pub fn trace_payload(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
        payload: impl Serialize,
    ) {
        emit(
            TraceLevel::Trace,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            Some(payload),
            true,
        );
    }

    pub fn debug(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
    ) {
        emit::<()>(
            TraceLevel::Debug,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            None,
            true,
        );
    }

    pub fn debug_payload(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
        payload: impl Serialize,
    ) {
        emit(
            TraceLevel::Debug,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            Some(payload),
            true,
        );
    }

    pub fn info(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
    ) {
        emit::<()>(
            TraceLevel::Info,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            None,
            true,
        );
    }

    pub fn info_payload(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
        payload: impl Serialize,
    ) {
        emit(
            TraceLevel::Info,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            Some(payload),
            true,
        );
    }

    pub fn warn(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
    ) {
        emit::<()>(
            TraceLevel::Warn,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            None,
            true,
        );
    }

    pub fn warn_payload(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
        payload: impl Serialize,
    ) {
        emit(
            TraceLevel::Warn,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            Some(payload),
            true,
        );
    }

    pub fn error(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
    ) {
        emit::<()>(
            TraceLevel::Error,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            None,
            true,
        );
    }

    pub fn error_payload(
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
        payload: impl Serialize,
    ) {
        emit(
            TraceLevel::Error,
            message_en,
            message_zh,
            gadget,
            unit,
            source_type,
            Some(payload),
            true,
        );
    }

    fn emit<T: Serialize>(
        level: TraceLevel,
        message_en: &str,
        message_zh: Option<&str>,
        gadget: &str,
        unit: &str,
        source_type: Option<&str>,
        payload: Option<T>,
        timestamp: bool,
    ) {
        // These two variables will be passed to the log server, but will not be output to the console.
        // 这两个变量会传递给日志服务器，但不会输出给控制台。
        let is_zh_cn = super::is_zh_cn();
        let _utc = || {
            if timestamp {
                Some(format!("{:?}", chrono::Utc::now().naive_utc()))
            } else {
                None
            }
        };

        let message = if is_zh_cn && message_zh.is_some() {
            message_zh.unwrap()
        } else {
            message_en
        };
        let payload = || payload.map(|p| serde_json::to_string(&p).unwrap());

        match level {
            TraceLevel::Trace => {
                tracing::trace!(target: "ostp", message, gadget, unit, source_type, payload = payload())
            }
            TraceLevel::Debug => {
                tracing::debug!(target: "ostp", message, gadget, unit, source_type, payload = payload())
            }
            TraceLevel::Info => {
                tracing::info!(target: "ostp", message, gadget, unit, source_type, payload = payload())
            }
            TraceLevel::Warn => {
                tracing::warn!(target: "ostp", message, gadget, unit, source_type, payload = payload())
            }
            TraceLevel::Error => {
                tracing::error!(target: "ostp", message, gadget, unit, source_type, payload = payload())
            }
        }
    }
}

use tracing_subscriber::fmt::{format::FmtSpan, time::ChronoLocal};

#[cfg(debug_assertions)]
pub fn install_default() {
    #[cfg(windows)]
    nu_ansi_term::enable_ansi_support().ok();

    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::new("%m-%d %H:%M:%S".into()))
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .with_thread_names(true)
        .init();
}

#[cfg(not(debug_assertions))]
pub fn install_default() {
    #[cfg(windows)]
    nu_ansi_term::enable_ansi_support().ok();

    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::new("%m-%d %H:%M:%S".into()))
        .with_span_events(FmtSpan::FULL)
        .with_thread_names(true)
        .init();
}

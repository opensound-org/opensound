#[derive(Debug)]
pub enum TraceLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub struct TraceData {
    level: TraceLevel,
    module: String,
    source: Option<String>,
    message: String,
    payload: Option<String>,
}

#[derive(Debug)]
pub struct EventData {
    _module: String,
    _source: String,
    _payload: String,
}

#[derive(Debug)]
pub enum Tracing {
    Trace(TraceData),
    Event(EventData),
}

pub fn emit(tracing: Tracing) {
    match tracing {
        Tracing::Trace(data) => {
            let module = data.module.as_str();
            let source = data.source.as_ref();
            let message = data.message.as_str();
            let payload = data.payload.as_ref();

            match data.level {
                TraceLevel::Trace => tracing::trace!(module, source, message, payload),
                TraceLevel::Debug => tracing::debug!(module, source, message, payload),
                TraceLevel::Info => tracing::info!(module, source, message, payload),
                TraceLevel::Warn => tracing::warn!(module, source, message, payload),
                TraceLevel::Error => tracing::error!(module, source, message, payload),
            }
        }
        Tracing::Event(_) => todo!(),
    }
}

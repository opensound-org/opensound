use crate::common::token::token_to_gadget_id;
use http::StatusCode;
use serde_json::{json, Value};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
    sync::{
        mpsc::UnboundedSender,
        oneshot::{self, Receiver, Sender},
        RwLock,
    },
    time::{timeout, Duration},
};

trait Resp {
    type R;
    fn resp(self, r: Self::R);
}

struct Common {
    pub endpoint: String,
    pub src_addr: SocketAddr,
    pub src_id: Option<String>,
}

struct GetQuery {
    pub common: Common,
    pub query: Option<HashMap<String, String>>,
    sender: Sender<(Option<String>, StatusCode)>,
}

impl Resp for GetQuery {
    type R = (Option<String>, StatusCode);

    fn resp(self, r: Self::R) {
        self.sender.send(r).ok();
    }
}

struct PostJSON {
    pub common: Common,
    pub payload: Value,
    sender: Sender<(Option<Value>, StatusCode)>,
}

impl Resp for PostJSON {
    type R = (Option<Value>, StatusCode);

    fn resp(self, r: Self::R) {
        self.sender.send(r).ok();
    }
}

struct PostText {
    pub common: Common,
    pub payload: String,
    sender: Sender<(Option<String>, StatusCode)>,
}

impl Resp for PostText {
    type R = (Option<String>, StatusCode);

    fn resp(self, r: Self::R) {
        self.sender.send(r).ok();
    }
}

struct PostBinary {
    pub common: Common,
    pub payload: Vec<u8>,
    sender: Sender<(Option<Vec<u8>>, StatusCode)>,
}

impl Resp for PostBinary {
    type R = (Option<Vec<u8>>, StatusCode);

    fn resp(self, r: Self::R) {
        self.sender.send(r).ok();
    }
}

enum Func {
    GetQuery(GetQuery),
    PostJSON(PostJSON),
    PostText(PostText),
    PostBinary(PostBinary),
}

impl Func {
    fn get_query(
        endpoint: &str,
        src_addr: SocketAddr,
        src_id: Option<&str>,
        query: Option<HashMap<String, String>>,
    ) -> (Self, Receiver<(Option<String>, StatusCode)>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::GetQuery(GetQuery {
                common: Common {
                    endpoint: endpoint.into(),
                    src_addr,
                    src_id: src_id.map(From::from),
                },
                query,
                sender,
            }),
            receiver,
        )
    }

    fn post_json(
        endpoint: &str,
        src_addr: SocketAddr,
        src_id: Option<&str>,
        payload: Value,
    ) -> (Self, Receiver<(Option<Value>, StatusCode)>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::PostJSON(PostJSON {
                common: Common {
                    endpoint: endpoint.into(),
                    src_addr,
                    src_id: src_id.map(From::from),
                },
                payload,
                sender,
            }),
            receiver,
        )
    }

    fn post_text(
        endpoint: &str,
        src_addr: SocketAddr,
        src_id: Option<&str>,
        payload: &str,
    ) -> (Self, Receiver<(Option<String>, StatusCode)>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::PostText(PostText {
                common: Common {
                    endpoint: endpoint.into(),
                    src_addr,
                    src_id: src_id.map(From::from),
                },
                payload: payload.into(),
                sender,
            }),
            receiver,
        )
    }

    fn post_binary(
        endpoint: &str,
        src_addr: SocketAddr,
        src_id: Option<&str>,
        payload: &[u8],
    ) -> (Self, Receiver<(Option<Vec<u8>>, StatusCode)>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self::PostBinary(PostBinary {
                common: Common {
                    endpoint: endpoint.into(),
                    src_addr,
                    src_id: src_id.map(From::from),
                },
                payload: payload.into(),
                sender,
            }),
            receiver,
        )
    }
}

struct FuncGateway(Arc<RwLock<HashMap<String, UnboundedSender<Func>>>>);

impl FuncGateway {
    async fn get_query(
        &self,
        id: &str,
        endpoint: &str,
        src_addr: SocketAddr,
        token: Option<&str>,
        query: Option<HashMap<String, String>>,
    ) -> (Option<String>, StatusCode) {
        match self.get_sender(id).await {
            Some(sender) => {
                let src_id = if let Some(token) = token {
                    if let Ok(id) = token_to_gadget_id(token) {
                        Some(id)
                    } else {
                        return (None, StatusCode::FORBIDDEN);
                    }
                } else {
                    None
                };
                let src_id = src_id.as_deref();
                let (func, receiver) = Func::get_query(endpoint, src_addr, src_id, query);

                // __todo__: Firewall & Existence & AccessControlRegistry (src_addr::src_id -> id/endpoint)

                if sender.send(func).is_ok() {
                    match timeout(Self::get_timeout_dur(), receiver).await {
                        Ok(Ok(resp)) => resp,
                        Ok(Err(_)) => (None, StatusCode::BAD_GATEWAY),
                        Err(_) => (None, StatusCode::GATEWAY_TIMEOUT),
                    }
                } else {
                    (None, StatusCode::NOT_FOUND)
                }
            }
            None => (None, StatusCode::NOT_FOUND),
        }
    }

    async fn post_json(
        &self,
        id: &str,
        endpoint: &str,
        src_addr: SocketAddr,
        token: Option<&str>,
        payload: Value,
    ) -> (Option<Value>, StatusCode) {
        match self.get_sender(id).await {
            Some(sender) => {
                let src_id = if let Some(token) = token {
                    if let Ok(id) = token_to_gadget_id(token) {
                        Some(id)
                    } else {
                        return (None, StatusCode::FORBIDDEN);
                    }
                } else {
                    None
                };
                let src_id = src_id.as_deref();
                let (func, receiver) = Func::post_json(endpoint, src_addr, src_id, payload);

                // __todo__: Firewall & Existence & AccessControlRegistry (src_addr::src_id -> id/endpoint)

                if sender.send(func).is_ok() {
                    match timeout(Self::get_timeout_dur(), receiver).await {
                        Ok(Ok(resp)) => resp,
                        Ok(Err(_)) => (None, StatusCode::BAD_GATEWAY),
                        Err(_) => (None, StatusCode::GATEWAY_TIMEOUT),
                    }
                } else {
                    (None, StatusCode::NOT_FOUND)
                }
            }
            None => (None, StatusCode::NOT_FOUND),
        }
    }

    async fn post_text(
        &self,
        id: &str,
        endpoint: &str,
        src_addr: SocketAddr,
        token: Option<&str>,
        payload: &str,
    ) -> (Option<String>, StatusCode) {
        match self.get_sender(id).await {
            Some(sender) => {
                let src_id = if let Some(token) = token {
                    if let Ok(id) = token_to_gadget_id(token) {
                        Some(id)
                    } else {
                        return (None, StatusCode::FORBIDDEN);
                    }
                } else {
                    None
                };
                let src_id = src_id.as_deref();
                let (func, receiver) = Func::post_text(endpoint, src_addr, src_id, payload);

                // __todo__: Firewall & Existence & AccessControlRegistry (src_addr::src_id -> id/endpoint)

                if sender.send(func).is_ok() {
                    match timeout(Self::get_timeout_dur(), receiver).await {
                        Ok(Ok(resp)) => resp,
                        Ok(Err(_)) => (None, StatusCode::BAD_GATEWAY),
                        Err(_) => (None, StatusCode::GATEWAY_TIMEOUT),
                    }
                } else {
                    (None, StatusCode::NOT_FOUND)
                }
            }
            None => (None, StatusCode::NOT_FOUND),
        }
    }

    async fn post_binary(
        &self,
        id: &str,
        endpoint: &str,
        src_addr: SocketAddr,
        token: Option<&str>,
        payload: &[u8],
    ) -> (Option<Vec<u8>>, StatusCode) {
        match self.get_sender(id).await {
            Some(sender) => {
                let src_id = if let Some(token) = token {
                    if let Ok(id) = token_to_gadget_id(token) {
                        Some(id)
                    } else {
                        return (None, StatusCode::FORBIDDEN);
                    }
                } else {
                    None
                };
                let src_id = src_id.as_deref();
                let (func, receiver) = Func::post_binary(endpoint, src_addr, src_id, payload);

                // __todo__: Firewall & Existence & AccessControlRegistry (src_addr::src_id -> id/endpoint)

                if sender.send(func).is_ok() {
                    match timeout(Self::get_timeout_dur(), receiver).await {
                        Ok(Ok(resp)) => resp,
                        Ok(Err(_)) => (None, StatusCode::BAD_GATEWAY),
                        Err(_) => (None, StatusCode::GATEWAY_TIMEOUT),
                    }
                } else {
                    (None, StatusCode::NOT_FOUND)
                }
            }
            None => (None, StatusCode::NOT_FOUND),
        }
    }

    async fn get_sender(&self, id: &str) -> Option<UnboundedSender<Func>> {
        self.0.read().await.get(id).cloned()
    }

    #[cfg(debug_assertions)]
    fn get_timeout_dur() -> Duration {
        Duration::from_secs_f64(30.0)
    }

    #[cfg(not(debug_assertions))]
    fn get_timeout_dur() -> Duration {
        Duration::from_secs_f64(3.0)
    }
}

pub struct SysCtrl;

impl SysCtrl {
    pub fn http_index(backend: &str) -> String {
        format!("OpenSound HttpServer({}) is Up & Running!", backend)
    }

    pub fn hello() -> &'static str {
        "Hello, world!"
    }

    pub fn version() -> Value {
        json!({
            "version": crate::VERSION
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(SysCtrl::version().to_string(), "{\"version\":\"v0.0.6\"}");
    }
}

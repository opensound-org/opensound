use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use axum::{extract::State, response::Json, routing::get, Router};
use futures::FutureExt;
use serde_json::Value;
use std::{
    future::{Future, IntoFuture},
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;

const NAME: &'static str = "Axum";

async fn index() -> String {
    SysCtrl::http_index(NAME)
}

async fn hello() -> &'static str {
    SysCtrl::hello()
}

async fn version() -> Json<Value> {
    Json(SysCtrl::version())
}

async fn shutdown(State(state): State<Option<SysCtrl>>) -> &'static str {
    state.unwrap().trigger_shutdown()
}

async fn reboot(State(state): State<Option<SysCtrl>>) -> &'static str {
    state.unwrap().trigger_reboot()
}

async fn ignite_internal(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let mut router = Router::new()
        .route("/", get(index))
        .route("/api/v1/sys/hello", get(hello))
        .route("/api/v1/sys/version", get(version));

    if ctrl.is_some() {
        router = router
            .route("/api/v1/sys/shutdown", get(shutdown))
            .route("/api/v1/sys/reboot", get(reboot));
    }

    let router = router.with_state(ctrl);
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0))).await?;
    let addr = listener.local_addr().unwrap();
    let fut = axum::serve(listener, router)
        .with_graceful_shutdown(graceful_shutdown)
        .into_future();

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port, ctrl, graceful_shutdown).await)
}

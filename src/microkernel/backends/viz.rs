use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use futures::FutureExt;
use serde_json::Value;
use std::{
    future::{Future, IntoFuture},
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;
use viz::{
    serve,
    types::{Json, State},
    IntoHandler, Result, Router,
};

const NAME: &'static str = "Viz";

async fn index() -> Result<String> {
    Ok(SysCtrl::http_index(NAME))
}

async fn hello() -> Result<&'static str> {
    Ok(SysCtrl::hello())
}

async fn version() -> Result<Json<Value>> {
    Ok(Json(SysCtrl::version()))
}

async fn shutdown(state: State<SysCtrl>) -> Result<&'static str> {
    Ok(state.trigger_shutdown())
}

async fn reboot(state: State<SysCtrl>) -> Result<&'static str> {
    Ok(state.trigger_reboot())
}

async fn ignite_internal(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let mut sys = Router::new()
        .get("hello", hello.into_handler())
        .get("version", version.into_handler());

    if let Some(ctrl) = ctrl {
        sys = sys
            .get("shutdown", shutdown.into_handler())
            .get("reboot", reboot.into_handler())
            .with(State::new(ctrl));
    }

    let router = Router::new()
        .get("/", index.into_handler())
        .nest("api/v1/sys", sys);
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0))).await?;
    let addr = listener.local_addr()?;
    let fut = serve(listener, router)
        .signal(graceful_shutdown)
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

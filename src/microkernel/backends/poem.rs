use super::super::reqres::{EmbedStatics, SysCtrl};
use crate::common::CommonFut;
use futures::FutureExt;
use poem::{
    endpoint::EmbeddedFilesEndpoint,
    get, handler,
    listener::{Acceptor, Listener, TcpListener},
    web::{Data, Json},
    EndpointExt, Route, Server,
};
use serde_json::Value;
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

const NAME: &'static str = "Poem";

#[handler]
async fn index() -> String {
    SysCtrl::http_index(NAME)
}

#[handler]
async fn hello() -> &'static str {
    SysCtrl::hello()
}

#[handler]
async fn version() -> Json<Value> {
    Json(SysCtrl::version())
}

#[handler]
async fn shutdown(data: Data<&SysCtrl>) -> &'static str {
    data.0.trigger_shutdown()
}

#[handler]
async fn reboot(data: Data<&SysCtrl>) -> &'static str {
    data.0.trigger_reboot()
}

async fn ignite_internal(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let mut route = Route::new()
        .at("/", get(index))
        .nest(
            "/statics",
            get(EmbeddedFilesEndpoint::<EmbedStatics>::new()),
        )
        .at("/api/v1/sys/hello", get(hello))
        .at("/api/v1/sys/version", get(version));

    if ctrl.is_some() {
        route = route
            .at("/api/v1/sys/shutdown", get(shutdown))
            .at("/api/v1/sys/reboot", get(reboot));
    }

    let route = route.data_opt(ctrl);
    let acceptor = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))
        .into_acceptor()
        .await?;
    let addr = *acceptor.local_addr()[0].0.as_socket_addr().unwrap();
    let fut = Server::new_with_acceptor(acceptor).run_with_graceful_shutdown(
        route,
        graceful_shutdown,
        Some(Duration::from_secs(1)),
    );

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port, ctrl, graceful_shutdown).await)
}

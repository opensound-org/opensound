use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use futures::FutureExt;
use poem::{
    get, handler,
    listener::{Acceptor, Listener, TcpListener},
    web::Json,
    Route, Server,
};
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr};

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

async fn ignite_internal(port: Option<u16>) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let route = Route::new()
        .at("/", get(index))
        .at("/api/v1/hello", get(hello))
        .at("/api/v1/version", get(version));
    let acceptor = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))
        .into_acceptor()
        .await?;
    let addr = *acceptor.local_addr()[0].0.as_socket_addr().unwrap();
    let fut = Server::new_with_acceptor(acceptor).run(route);

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}

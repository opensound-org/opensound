use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use futures::FutureExt;
use salvo::prelude::*;
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr};

const NAME: &'static str = "Salvo";

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
    let router = Router::new().get(index).push(
        Router::with_path("api/v1/sys")
            .push(Router::with_path("hello").get(hello))
            .push(Router::with_path("version").get(version)),
    );
    let acceptor = TcpListener::new((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))
        .try_bind()
        .await?;
    let addr = acceptor.local_addr()?;
    let fut = Server::new(acceptor).serve(router);

    Ok((addr, async move { Ok(fut.await) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}

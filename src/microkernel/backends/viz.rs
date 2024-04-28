use super::super::reqres::SysApi;
use crate::common::CommonFut;
use futures::FutureExt;
use serde_json::Value;
use std::{
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;
use viz::{serve, types::Json, IntoHandler, Result, Router};

const NAME: &'static str = "Viz";

async fn index() -> Result<String> {
    Ok(SysApi::index(NAME))
}

async fn hello() -> Result<&'static str> {
    Ok(SysApi::hello())
}

async fn version() -> Result<Json<Value>> {
    Ok(Json(SysApi::version()))
}

async fn ignite_internal(port: Option<u16>) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let router = Router::new().get("/", index.into_handler()).nest(
        "api/v1/sys",
        Router::new()
            .get("hello", hello.into_handler())
            .get("version", version.into_handler()),
    );
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0))).await?;
    let addr = listener.local_addr()?;
    let fut = serve(listener, router).into_future();

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}

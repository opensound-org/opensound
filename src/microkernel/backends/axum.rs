use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use axum::{response::Json, routing::get, Router};
use futures::FutureExt;
use serde_json::Value;
use std::{
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;

const NAME: &'static str = "Axum";

async fn index() -> String {
    SysCtrl::index(NAME)
}

async fn hello() -> &'static str {
    SysCtrl::hello()
}

async fn version() -> Json<Value> {
    Json(SysCtrl::version())
}

async fn ignite_internal(port: Option<u16>) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let router = Router::new()
        .route("/", get(index))
        .route("/api/v1/sys/hello", get(hello))
        .route("/api/v1/sys/version", get(version));
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0))).await?;
    let addr = listener.local_addr().unwrap();
    let fut = axum::serve(listener, router).into_future();

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}

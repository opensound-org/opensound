use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use actix_web::{get, web::Json, App, HttpServer};
use futures::FutureExt;
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr};

const NAME: &'static str = "ActixWeb";

#[get("/")]
async fn index() -> String {
    SysCtrl::http_index(NAME)
}

#[get("/api/v1/sys/hello")]
async fn hello() -> &'static str {
    SysCtrl::hello()
}

#[get("/api/v1/sys/version")]
async fn version() -> Json<Value> {
    Json(SysCtrl::version())
}

async fn ignite_internal(port: Option<u16>) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let server = HttpServer::new(|| App::new().service(index).service(hello).service(version))
        .bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))?
        .disable_signals();
    let addr = server.addrs()[0];
    let fut = server.run();

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}
use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use futures::FutureExt;
use rocket::{
    config::{LogLevel, Shutdown},
    fairing::AdHoc,
    get, routes,
    serde::json::Json,
    Config,
};
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::sync::oneshot;

const NAME: &'static str = "Rocket";

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
    let (addr_sender, addr_receiver) = oneshot::channel();
    let (err_sender, err_receiver) = oneshot::channel();
    let launch = rocket::custom(Config {
        address: Ipv4Addr::UNSPECIFIED.into(),
        port: port.unwrap_or(0),
        shutdown: Shutdown {
            ctrlc: false,
            ..Default::default()
        },
        log_level: LogLevel::Off,
        ..Default::default()
    })
    .mount("/", routes![index, hello, version])
    .attach(AdHoc::on_liftoff("Bind", |rocket| {
        Box::pin(async move {
            let config = rocket.config();
            let addr = SocketAddr::new(config.address, config.port);

            addr_sender.send(addr).unwrap();
        })
    }))
    .ignite()
    .await?
    .launch();
    let fut = tokio::spawn(async move {
        if let Err(err) = launch.await {
            err_sender.send(err).ok();
        }
    });

    let addr = tokio::select! {
        Ok(addr) = addr_receiver => addr,
        Ok(err) = err_receiver => return Err(err)?,
    };

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}
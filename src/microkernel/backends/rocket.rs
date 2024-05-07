use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use futures::FutureExt;
use rocket::{
    config::{LogLevel, Shutdown},
    fairing::AdHoc,
    get, routes,
    serde::json::Json,
    Config, State,
};
use serde_json::Value;
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
};
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

#[get("/api/v1/sys/shutdown")]
async fn shutdown(state: &State<SysCtrl>) -> &'static str {
    state.trigger_shutdown()
}

#[get("/api/v1/sys/reboot")]
async fn reboot(state: &State<SysCtrl>) -> &'static str {
    state.trigger_reboot()
}

async fn ignite_internal(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let (addr_sender, addr_receiver) = oneshot::channel();
    let (err_sender, err_receiver) = oneshot::channel();
    let mut build = rocket::custom(Config {
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
    }));

    if let Some(ctrl) = ctrl {
        build = build.mount("/", routes![shutdown, reboot]).manage(ctrl);
    }

    let ignite = build.ignite().await?;
    let shutdown = ignite.shutdown();
    let launch = ignite.launch();
    let fut = tokio::spawn(async move {
        if let Err(err) = launch.await {
            err_sender.send(err).ok();
        }
    });

    let addr = tokio::select! {
        Ok(addr) = addr_receiver => addr,
        Ok(err) = err_receiver => return Err(err)?,
    };

    tokio::spawn(async move {
        graceful_shutdown.await;
        shutdown.notify();
    });

    Ok((addr, async move { Ok(fut.await?) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port, ctrl, graceful_shutdown).await)
}

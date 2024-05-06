use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use actix_web::{
    get,
    web::{Data, Json},
    App, HttpServer,
};
use futures::FutureExt;
use serde_json::Value;
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
};

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

#[get("/api/v1/sys/shutdown")]
async fn shutdown(data: Data<SysCtrl>) -> &'static str {
    data.trigger_shutdown()
}

#[get("/api/v1/sys/reboot")]
async fn reboot(data: Data<SysCtrl>) -> &'static str {
    data.trigger_reboot()
}

async fn ignite_internal(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let server = HttpServer::new(move || {
        let mut app = App::new().service(index).service(hello).service(version);

        if let Some(ctrl) = ctrl.clone() {
            app = app
                .app_data(Data::new(ctrl))
                .service(shutdown)
                .service(reboot);
        }

        app
    })
    .bind((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))?
    .disable_signals()
    .shutdown_timeout(1);
    let addr = server.addrs()[0];
    let fut = server.run();
    let handle = fut.handle();

    tokio::spawn(async move {
        graceful_shutdown.await;

        // There is no need for await, just await JoinHanle externally and that's it.
        // 不需要await这玩意儿，直接在外部await JoinHanle就完事儿了
        let _ = handle.stop(true);
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

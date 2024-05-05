use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use futures::FutureExt;
use salvo::prelude::*;
use serde_json::Value;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::sync::oneshot::Receiver;

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

#[handler]
async fn shutdown(depot: &Depot) -> &'static str {
    depot.obtain::<SysCtrl>().unwrap().trigger_shutdown()
}

#[handler]
async fn reboot(depot: &Depot) -> &'static str {
    depot.obtain::<SysCtrl>().unwrap().trigger_reboot()
}

async fn ignite_internal(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: Receiver<()>,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let mut sys = Router::with_path("api/v1/sys")
        .push(Router::with_path("hello").get(hello))
        .push(Router::with_path("version").get(version));

    if let Some(ctrl) = ctrl {
        sys = sys
            .hoop(affix::inject(ctrl))
            .push(Router::with_path("shutdown").get(shutdown))
            .push(Router::with_path("reboot").get(reboot));
    }

    let router = Router::new().get(index).push(sys);
    let acceptor = TcpListener::new((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))
        .try_bind()
        .await?;
    let addr = acceptor.local_addr()?;

    // Here is a workaround for the Salvo graceful shutdown bug!
    // 这里是Salvo优雅停机bug的workaround！
    // https://github.com/salvo-rs/salvo/issues/764
    let fut = tokio::spawn(Server::new(acceptor).serve(router));
    let handle = fut.abort_handle();

    // Here is a workaround for the Salvo graceful shutdown bug!
    // 这里是Salvo优雅停机bug的workaround！
    // https://github.com/salvo-rs/salvo/issues/764
    tokio::spawn(async move {
        graceful_shutdown.await.ok();
        handle.abort();
    });

    // Here is a workaround for the Salvo graceful shutdown bug!
    // 这里是Salvo优雅停机bug的workaround！
    // https://github.com/salvo-rs/salvo/issues/764
    Ok((addr, async move { Ok(fut.await.unwrap_or(())) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: Receiver<()>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port, ctrl, graceful_shutdown).await)
}

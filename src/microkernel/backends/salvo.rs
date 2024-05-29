use super::super::reqres::{EmbedStatics, SysCtrl};
use crate::common::CommonFut;
use futures::FutureExt;
use salvo::{prelude::*, serve_static::static_embed};
use serde_json::Value;
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

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
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let statics = Router::with_path("statics/<**>")
        .get(static_embed::<EmbedStatics>().defaults("index.html"));
    let mut sys = Router::with_path("api/v1/sys")
        .push(Router::with_path("hello").get(hello))
        .push(Router::with_path("version").get(version));

    if let Some(ctrl) = ctrl {
        sys = sys
            .hoop(affix::inject(ctrl))
            .push(Router::with_path("shutdown").get(shutdown))
            .push(Router::with_path("reboot").get(reboot));
    }

    let router = Router::new().get(index).push(statics).push(sys);
    let acceptor = TcpListener::new((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))
        .try_bind()
        .await?;
    let addr = acceptor.local_addr()?;
    let server = Server::new(acceptor);
    let handle = server.handle();
    let fut = server.serve(router);

    tokio::spawn(async move {
        graceful_shutdown.await;
        handle.stop_graceful(Some(Duration::from_secs(1)));
    });

    Ok((addr, async move { Ok(fut.await) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port, ctrl, graceful_shutdown).await)
}

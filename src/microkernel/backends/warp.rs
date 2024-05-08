use super::super::reqres::SysCtrl;
use crate::common::CommonFut;
use futures::FutureExt;
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
};
use warp::{http::StatusCode, reply::json, Filter};

const NAME: &'static str = "Warp";

async fn ignite_internal(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let (ctrl_s, ctrl_r) = (ctrl.clone(), ctrl);
    let index = warp::path!().then(|| async move { SysCtrl::http_index(NAME) });
    let hello =
        warp::path!("api" / "v1" / "sys" / "hello").then(|| async move { SysCtrl::hello() });
    let version = warp::path!("api" / "v1" / "sys" / "version")
        .then(|| async move { json(&SysCtrl::version()) });
    let shutdown = warp::path!("api" / "v1" / "sys" / "shutdown").then(move || {
        let ctrl = ctrl_s.clone();
        async move {
            match ctrl {
                Some(ctrl) => Ok(ctrl.trigger_shutdown()),
                None => Err(StatusCode::NOT_FOUND),
            }
        }
    });
    let reboot = warp::path!("api" / "v1" / "sys" / "reboot").then(move || {
        let ctrl = ctrl_r.clone();
        async move {
            match ctrl.clone() {
                Some(ctrl) => Ok(ctrl.trigger_reboot()),
                None => Err(StatusCode::NOT_FOUND),
            }
        }
    });
    let routes = warp::get().and(index.or(hello).or(version).or(shutdown).or(reboot));
    let (addr, fut) = warp::serve(routes).try_bind_with_graceful_shutdown(
        (Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)),
        graceful_shutdown,
    )?;

    Ok((addr, async move { Ok(fut.await) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
    ctrl: Option<SysCtrl>,
    graceful_shutdown: impl Future<Output = ()> + Send + 'static,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port, ctrl, graceful_shutdown).await)
}

use super::super::reqres::SysApi;
use crate::common::CommonFut;
use futures::FutureExt;
use std::net::{Ipv4Addr, SocketAddr};
use warp::{reply::json, Filter};

const NAME: &'static str = "Warp";

async fn ignite_internal(port: Option<u16>) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let index = warp::path!().map(|| SysApi::index(NAME));
    let hello = warp::path!("api" / "v1" / "sys" / "hello").map(|| SysApi::hello());
    let version = warp::path!("api" / "v1" / "sys" / "version").map(|| json(&SysApi::version()));
    let routes = warp::get().and(index.or(hello).or(version));
    let (addr, fut) =
        warp::serve(routes).try_bind_ephemeral((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)))?;

    Ok((addr, async move { Ok(fut.await) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}

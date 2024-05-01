use super::super::reqres::SysCtrl;
use crate::common::{CommonFut, CommonRes};
use futures::FutureExt;
use ntex::web::{get, types::Json, App, HttpServer};
use serde_json::Value;
use socket2::{Domain, Socket, Type};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, ToSocketAddrs};
use tokio::{sync::oneshot, task::spawn_blocking};

const NAME: &'static str = "Ntex";

#[get("/")]
async fn index() -> String {
    SysCtrl::index(NAME)
}

#[get("/api/v1/sys/hello")]
async fn hello() -> &'static str {
    SysCtrl::hello()
}

#[get("/api/v1/sys/version")]
async fn version() -> Json<Value> {
    Json(SysCtrl::version())
}

#[ntex::main]
async fn launch(listener: TcpListener, shutdown: oneshot::Receiver<()>) -> CommonRes {
    let fut = HttpServer::new(|| App::new().service(index).service(hello).service(version))
        .listen(listener)?
        .disable_signals()
        .run();

    // The implementation of "Graceful Shutdown" needs to be optimized here.
    // 此处需要优化“优雅停机”的实现。
    tokio::select! {
        _ = shutdown => Ok(()),
        res = fut => Ok(res?),
    }
}

// Copied from ntex-server/src/net/builder.rs
// 拷贝自ntex-server/src/net/builder.rs
fn create_tcp_listener(addr: SocketAddr, backlog: i32) -> std::io::Result<TcpListener> {
    let builder = match addr {
        SocketAddr::V4(_) => Socket::new(Domain::IPV4, Type::STREAM, None)?,
        SocketAddr::V6(_) => Socket::new(Domain::IPV6, Type::STREAM, None)?,
    };

    // On Windows, this allows rebinding sockets which are actively in use,
    // which allows “socket hijacking”, so we explicitly don't set it here.
    // https://docs.microsoft.com/en-us/windows/win32/winsock/using-so-reuseaddr-and-so-exclusiveaddruse
    #[cfg(not(windows))]
    builder.set_reuse_address(true)?;

    builder.bind(&socket2::SockAddr::from(addr))?;
    builder.listen(backlog)?;
    Ok(TcpListener::from(builder))
}

// Copied from ntex/src/web/server.rs, with minor changes (backlog default value 1024)
// 拷贝自ntex/src/web/server.rs，有少量改动（backlog默认值1024）
fn bind2<A: ToSocketAddrs>(addr: A, backlog: Option<i32>) -> std::io::Result<Vec<TcpListener>> {
    let mut err = None;
    let mut succ = false;
    let mut sockets = Vec::new();
    for addr in addr.to_socket_addrs()? {
        match create_tcp_listener(addr, backlog.unwrap_or(1024)) {
            Ok(lst) => {
                succ = true;
                sockets.push(lst);
            }
            Err(e) => err = Some(e),
        }
    }

    if !succ {
        if let Some(e) = err.take() {
            Err(e)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot bind to address.",
            ))
        }
    } else {
        Ok(sockets)
    }
}

async fn ignite_internal(port: Option<u16>) -> Result<(SocketAddr, CommonFut), anyhow::Error> {
    let listener = bind2((Ipv4Addr::UNSPECIFIED, port.unwrap_or(0)), None)?.remove(0);
    let addr = listener.local_addr()?;

    // The implementation of "Graceful Shutdown" needs to be optimized here.
    // 此处需要优化“优雅停机”的实现。
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let fut = async move {
        let _shutdown_sender = shutdown_sender;
        spawn_blocking(move || launch(listener, shutdown_receiver)).await
    };

    Ok((addr, async move { Ok(fut.await??) }.boxed()))
}

pub async fn ignite(
    port: Option<u16>,
) -> (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>) {
    (NAME, ignite_internal(port).await)
}

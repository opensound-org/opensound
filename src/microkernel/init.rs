use super::{
    backends,
    reqres::{SysCtrl, SysEvent, SysEventRecv},
};
use crate::common::{ostp, CommonRes};
use tokio::{
    sync::oneshot::{self, Sender},
    task::JoinHandle,
};

async fn _ws() -> CommonRes {
    use futures::{SinkExt, StreamExt};
    use tokio::net::TcpListener;
    use tokio_tungstenite::{accept_hdr_async, tungstenite::handshake::server::Request};

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Ok(mut websocket) = accept_hdr_async(socket, |req: &Request, res| {
                if req.uri().path() == "/test" {
                    Ok(res)
                } else {
                    Err(Default::default())
                }
            })
            .await
            {
                while let Some(Ok(msg)) = websocket.next().await {
                    // We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        websocket.send(msg).await.ok();
                    }
                }
            }
        });
    }
}

struct HttpServer {
    _port: u16,
    graceful_shutdown: Sender<()>,
    routine: JoinHandle<CommonRes>,
}

impl HttpServer {
    async fn launch(port: Option<u16>, ctrl: SysCtrl) -> Result<Self, anyhow::Error> {
        #[cfg(feature = "salvo")]
        let ignite = backends::salvo::ignite;
        #[cfg(feature = "poem")]
        let ignite = backends::poem::ignite;
        #[cfg(feature = "actix-web")]
        let ignite = backends::actix_web::ignite;
        #[cfg(feature = "axum")]
        let ignite = backends::axum::ignite;
        #[cfg(feature = "rocket")]
        let ignite = backends::rocket::ignite;
        #[cfg(feature = "warp")]
        let ignite = backends::warp::ignite;
        #[cfg(feature = "viz")]
        let ignite = backends::viz::ignite;
        #[cfg(feature = "ntex")]
        let ignite = backends::ntex::ignite;

        let ctrl = Self::wrap_ctrl(ctrl);
        let (s, r) = oneshot::channel();
        let graceful_shutdown = async move {
            r.await.ok();
        };
        let (backend, res) = ignite(port, ctrl, graceful_shutdown).await;
        match res {
            Ok((addr, future)) => {
                let message_en = format!("OpenSound HttpServer({}) launched at: {}", backend, addr);
                let message_zh = Some(format!(
                    "OpenSound HttpServer（{}）启动于：{}",
                    backend, addr
                ));
                let message_zh = message_zh.as_deref();
                ostp::emit::info(&message_en, message_zh, "MicroKernel", "HttpServer", None);

                let _port = addr.port();
                let graceful_shutdown = s;
                let routine = tokio::spawn(future);

                Ok(Self {
                    _port,
                    graceful_shutdown,
                    routine,
                })
            }
            Err(err) => {
                let message_en = format!("{} backend launch failed! Reason: {}", backend, err);
                let message_zh = Some(format!("{}后端启动失败！原因：{}", backend, err));
                let message_zh = message_zh.as_deref();
                ostp::emit::error(&message_en, message_zh, "MicroKernel", "HttpServer", None);
                Err(err)
            }
        }
    }

    #[cfg(debug_assertions)]
    fn wrap_ctrl(ctrl: SysCtrl) -> Option<SysCtrl> {
        Some(ctrl)
    }

    #[cfg(not(debug_assertions))]
    fn wrap_ctrl(_ctrl: SysCtrl) -> Option<SysCtrl> {
        None
    }

    fn _get_port(&self) -> u16 {
        self._port
    }

    async fn graceful_shutdown(self) -> CommonRes {
        self.graceful_shutdown.send(()).ok();
        self.routine.await?
    }
}

pub struct MicroKernel {
    http_server: HttpServer,
    sys_event_recv: SysEventRecv,
}

impl MicroKernel {
    pub async fn launch(http_port: Option<u16>) -> Result<Self, anyhow::Error> {
        let (sys_ctrl, sys_event_recv) = SysCtrl::create_pair();
        let http_server = HttpServer::launch(http_port, sys_ctrl).await?;

        Ok(Self {
            http_server,
            sys_event_recv,
        })
    }

    #[cfg(debug_assertions)]
    pub const fn sys_ctrl_enabled(&self) -> bool {
        true
    }

    #[cfg(not(debug_assertions))]
    pub const fn sys_ctrl_enabled(&self) -> bool {
        false
    }

    pub fn _get_http_port(&self) -> u16 {
        self.http_server._get_port()
    }

    pub async fn sys_event(&mut self) -> SysEvent {
        self.sys_event_recv.recv().await
    }

    pub async fn graceful_shutdown(self) -> CommonRes {
        ostp::emit::info(
            "Performing Graceful Shutdown on HttpServer",
            Some("正在对HttpServer执行优雅停机"),
            "MicroKernel",
            "HttpServer",
            None,
        );
        self.http_server.graceful_shutdown().await
    }
}

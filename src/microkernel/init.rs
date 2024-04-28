use super::backends;
use crate::common::{ostp, CommonFut, CommonRes};
use std::net::SocketAddr;
use tokio::task::JoinHandle;

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

pub struct MicroKernel {
    _http_port: u16,
    http_server: JoinHandle<CommonRes>,
}

impl MicroKernel {
    pub async fn launch(port: Option<u16>) -> Result<Self, anyhow::Error> {
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

        let (_http_port, http_server) = Self::launch_backend(ignite(port).await)?;
        Ok(Self {
            _http_port,
            http_server,
        })
    }

    pub async fn join(self) -> CommonRes {
        self.http_server.await.unwrap()
    }

    fn launch_backend(
        (backend, res): (&'static str, Result<(SocketAddr, CommonFut), anyhow::Error>),
    ) -> Result<(u16, JoinHandle<CommonRes>), anyhow::Error> {
        match res {
            Ok((addr, future)) => {
                let message_en = format!("{} launched at: {}", backend, addr);
                let message_zh = Some(format!("{}启动于：{}", backend, addr));
                let message_zh = message_zh.as_deref();
                ostp::emit::info(&message_en, message_zh, "sys", "launch", None);

                let http_port = addr.port();
                let http_server = tokio::spawn(future);

                Ok((http_port, http_server))
            }
            Err(err) => {
                let message_en = format!("{} launch failed! Reason: {}", backend, err);
                let message_zh = Some(format!("{}启动失败！原因：{}", backend, err));
                let message_zh = message_zh.as_deref();
                ostp::emit::error(&message_en, message_zh, "sys", "launch", None);
                Err(err)
            }
        }
    }
}

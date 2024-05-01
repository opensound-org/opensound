use super::backends;
use crate::common::{ostp, CommonRes};
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

struct HttpServer {
    _port: u16,
    routine: JoinHandle<CommonRes>,
}

impl HttpServer {
    async fn launch(port: Option<u16>) -> Result<Self, anyhow::Error> {
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

        let (backend, res) = ignite(port).await;
        match res {
            Ok((addr, future)) => {
                let message_en = format!("OpenSound HttpServer({}) launched at: {}", backend, addr);
                let message_zh = Some(format!(
                    "OpenSound HttpServer（{}）启动于：{}",
                    backend, addr
                ));
                let message_zh = message_zh.as_deref();
                ostp::emit::info(&message_en, message_zh, "sys", "launch", None);

                let _port = addr.port();
                let routine = tokio::spawn(future);

                Ok(Self { _port, routine })
            }
            Err(err) => {
                let message_en = format!("{} backend launch failed! Reason: {}", backend, err);
                let message_zh = Some(format!("{}后端启动失败！原因：{}", backend, err));
                let message_zh = message_zh.as_deref();
                ostp::emit::error(&message_en, message_zh, "sys", "launch", None);
                Err(err)
            }
        }
    }
}

pub struct MicroKernel {
    http_server: HttpServer,
}

impl MicroKernel {
    pub async fn launch(http_port: Option<u16>) -> Result<Self, anyhow::Error> {
        let http_server = HttpServer::launch(http_port).await?;
        Ok(Self { http_server })
    }

    pub async fn join(self) -> CommonRes {
        self.http_server.routine.await.unwrap()
    }
}

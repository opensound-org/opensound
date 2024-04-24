use crate::common::{ostp, CommonRes};

#[allow(dead_code)]
mod reqres {
    use http::StatusCode;
    use serde_json::Value;
    use std::collections::HashMap;
    use tokio::sync::{
        mpsc::UnboundedSender,
        oneshot::{self, Receiver, Sender},
    };

    trait Resp {
        type R;
        fn resp(self, r: Self::R);
    }

    struct Common {
        pub id: String,
        pub endpoint: String,
        pub token: Option<String>,
    }

    struct GetQuery {
        pub common: Common,
        pub query: Option<HashMap<String, String>>,
        sender: Sender<(Option<String>, StatusCode)>,
    }

    impl Resp for GetQuery {
        type R = (Option<String>, StatusCode);

        fn resp(self, r: Self::R) {
            self.sender.send(r).ok();
        }
    }

    struct PostJSON {
        pub common: Common,
        pub payload: Value,
        sender: Sender<(Option<Value>, StatusCode)>,
    }

    impl Resp for PostJSON {
        type R = (Option<Value>, StatusCode);

        fn resp(self, r: Self::R) {
            self.sender.send(r).ok();
        }
    }

    struct PostText {
        pub common: Common,
        pub payload: String,
        sender: Sender<(Option<String>, StatusCode)>,
    }

    impl Resp for PostText {
        type R = (Option<String>, StatusCode);

        fn resp(self, r: Self::R) {
            self.sender.send(r).ok();
        }
    }

    struct PostBinary {
        pub common: Common,
        pub payload: Vec<u8>,
        sender: Sender<(Option<Vec<u8>>, StatusCode)>,
    }

    impl Resp for PostBinary {
        type R = (Option<Vec<u8>>, StatusCode);

        fn resp(self, r: Self::R) {
            self.sender.send(r).ok();
        }
    }

    enum Func {
        GetQuery(GetQuery),
        PostJSON(PostJSON),
        PostText(PostText),
        PostBinary(PostBinary),
    }

    impl Func {
        fn get_query(
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            query: Option<HashMap<String, String>>,
        ) -> (Self, Receiver<(Option<String>, StatusCode)>) {
            let (sender, receiver) = oneshot::channel();
            (
                Self::GetQuery(GetQuery {
                    common: Common {
                        id: id.into(),
                        endpoint: endpoint.into(),
                        token: token.map(From::from),
                    },
                    query,
                    sender,
                }),
                receiver,
            )
        }

        fn post_json(
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            payload: Value,
        ) -> (Self, Receiver<(Option<Value>, StatusCode)>) {
            let (sender, receiver) = oneshot::channel();
            (
                Self::PostJSON(PostJSON {
                    common: Common {
                        id: id.into(),
                        endpoint: endpoint.into(),
                        token: token.map(From::from),
                    },
                    payload,
                    sender,
                }),
                receiver,
            )
        }

        fn post_text(
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            payload: &str,
        ) -> (Self, Receiver<(Option<String>, StatusCode)>) {
            let (sender, receiver) = oneshot::channel();
            (
                Self::PostText(PostText {
                    common: Common {
                        id: id.into(),
                        endpoint: endpoint.into(),
                        token: token.map(From::from),
                    },
                    payload: payload.into(),
                    sender,
                }),
                receiver,
            )
        }

        fn post_binary(
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            payload: &[u8],
        ) -> (Self, Receiver<(Option<Vec<u8>>, StatusCode)>) {
            let (sender, receiver) = oneshot::channel();
            (
                Self::PostBinary(PostBinary {
                    common: Common {
                        id: id.into(),
                        endpoint: endpoint.into(),
                        token: token.map(From::from),
                    },
                    payload: payload.into(),
                    sender,
                }),
                receiver,
            )
        }
    }

    struct HttpReqRes(UnboundedSender<Func>);

    impl HttpReqRes {
        async fn get_query(
            &self,
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            query: Option<HashMap<String, String>>,
        ) -> (Option<String>, StatusCode) {
            let (func, receiver) = Func::get_query(id, endpoint, token, query);
            self.0.send(func).ok();
            receiver.await.unwrap()
        }

        async fn post_json(
            &self,
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            payload: Value,
        ) -> (Option<Value>, StatusCode) {
            let (func, receiver) = Func::post_json(id, endpoint, token, payload);
            self.0.send(func).ok();
            receiver.await.unwrap()
        }

        async fn post_text(
            &self,
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            payload: &str,
        ) -> (Option<String>, StatusCode) {
            let (func, receiver) = Func::post_text(id, endpoint, token, payload);
            self.0.send(func).ok();
            receiver.await.unwrap()
        }

        async fn post_binary(
            &self,
            id: &str,
            endpoint: &str,
            token: Option<&str>,
            payload: &[u8],
        ) -> (Option<Vec<u8>>, StatusCode) {
            let (func, receiver) = Func::post_binary(id, endpoint, token, payload);
            self.0.send(func).ok();
            receiver.await.unwrap()
        }
    }
}

/// MicroKernel entry
///
/// 微内核入口
pub async fn main() -> CommonRes {
    ostp::emit::debug(
        "Hello, world!",
        Some("你好，世界！"),
        "microkernel",
        "main",
        None,
    );

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

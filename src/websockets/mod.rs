use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};

pub mod event_loop;
pub mod handlers;
pub mod messages;

pub type WriteSink = futures::stream::SplitSink<
    WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    tungstenite::Message,
>;

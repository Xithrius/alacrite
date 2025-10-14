use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};

pub mod event_loop;
pub mod handle_message;
pub mod messages;

pub type WriteSink = futures::stream::SplitSink<
    WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    tungstenite::Message,
>;

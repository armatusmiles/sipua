use crate::transport::sip::Protocol;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub enum ConnectError {
    ServerClosedConnection,
}

pub trait RequestWriter {
    fn write(&self, r: parsip::Request) -> std::io::Result<()>;
    fn close(&self); // close connection
}

/// Calls if established connection with server is interrupted
pub type ErrorCb = fn(ConnectError);

/// The callback is async and may have a block operation
pub type ResponseCb = fn(parsip::Response, &dyn RequestWriter);

pub struct Client {
    protocol: Protocol,
    response_cb: ResponseCb,
    error_cb: ErrorCb,
}

impl Client {
    pub fn new(p: Protocol, rc: ResponseCb, ec: ErrorCb) -> Client {
        Client {
            protocol: p,
            response_cb: rc,
            error_cb: ec,
        }
    }

    pub async fn connect<A>(&self, addr: A) -> std::io::Result<()>
    where
        A: tokio::net::ToSocketAddrs,
    {
        let socket = UdpSocket::bind("0.0.0.0:0").await;
        socket.unwrap().connect(addr).await
    }
}
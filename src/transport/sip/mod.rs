use parsip;
use std;

#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
}

#[derive(Debug)]
pub enum ConnectError {
    ServerClosedConnection,
}

pub trait ResponseWriter {
    fn write(&self, r: parsip::Response) -> std::io::Result<()>;
    fn close(&self);
}

/// The callback is async and may have a block operation
pub type ResponseCb = fn(parsip::Response, &dyn ResponseWriter);

/// Calls if established connection with server is interrupted
pub type ErrorCb = fn(ConnectError);

pub struct Client {
    protocol: Protocol,
    response_cb: ResponseCb,
    error_cb: ErrorCb,
}

use tokio::net::UdpSocket;
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

pub struct SipServer {}

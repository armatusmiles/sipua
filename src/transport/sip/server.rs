pub trait ResponseWriter {
    fn write(&self, r: parsip::Response) -> std::io::Result<()>;
    fn close(&self); // close connection
}

pub struct SipServer {}

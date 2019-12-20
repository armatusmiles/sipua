use sipua::transport;

fn client_resp(r: parsip::Response, w: &dyn transport::sip::RequestWriter) {}

fn client_err(ce: transport::sip::ConnectError) {}

#[tokio::test]
async fn test_interface() {
    let sip_client =
        transport::sip::Client::new(transport::sip::Protocol::UDP, client_resp, client_err);
    sip_client.connect("127.0.0.1:7777").await;
}

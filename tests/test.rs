extern crate sipua;

const NUM_OF_HEADERS: usize = 32;

#[test]
fn dummy() {
    let mut headers = [sipua::parsip::EMPTY_HEADER; NUM_OF_HEADERS];
    let req = sipua::parsip::Request::new(&mut headers[..]);
    assert_eq!(req.version, None);
}

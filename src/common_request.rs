use hyper::{Body, Request, Response};

pub fn get_signature_from_request(request: &Request<Body>) -> String {
    let signature: String = String::from_utf8(
        request
            .headers()
            .get("signature")
            .unwrap()
            .as_bytes()
            .to_vec(),
    )
    .unwrap();

    signature
}

pub fn get_signature_from_response(response: &Response<Body>) -> String {
    let signature: String = String::from_utf8(
        response
            .headers()
            .get("signature")
            .unwrap()
            .as_bytes()
            .to_vec(),
    )
    .unwrap();

    signature
}

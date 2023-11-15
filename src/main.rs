use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use rand::RngCore;
use sha3::{Digest, Sha3_256};

pub type StdError = Box<dyn std::error::Error + Send + Sync>;

fn sha3_256_digest(data: &[u8]) -> Vec<u8> {
    let mut digest = Sha3_256::new();
    digest.update(data);

    let fin = digest.finalize();
    let buf: &[u8] = fin.as_slice();

    Vec::from(buf)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), StdError> {
    let address: SocketAddr = "0.0.0.0:8080".parse()?;

    let make_svc = make_service_fn(|_| async move {
        Ok::<_, Infallible>(service_fn(move |req| handle_request(req)))
    });

    let server = Server::bind(&address).serve(make_svc);
    server.await?;

    Ok(())
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, StdError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/hash/sha3-256/random") => {
            let mut buf = vec![0; 32];
            rand::thread_rng().fill_bytes(&mut buf);

            let digest = sha3_256_digest(&buf);

            let res = Response::builder()
                .header("content-type", "text/plain")
                .status(StatusCode::OK)
                .body(Body::from(hex::encode(digest)))
                .unwrap();

            Ok(res)
        }
        _ => {
            let not_found = Response::builder()
                .header("content-type", "text/plain")
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("not found"))
                .unwrap();

            Ok(not_found)
        }
    }
}

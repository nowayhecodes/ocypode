use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

struct RateLimiter {
    requests: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

impl RateLimiter {
    fn new() -> Self {
        RateLimiter {
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn allow(&self, addr: SocketAddr) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let count = requests.entry(addr).or_insert(0);
        if *count >= 5 {
            false
        } else {
            *count += 1;
            true
        }
    }
}

async fn service_handler(path: &str) -> Result<Response<Body>, hyper::Error> {
    match path {
        "/healthz" => Ok(Response::builder()
            .status(200)
            .body(Body::from("OK"))
            .unwrap()),
        "/service" => Ok(Response::new(Body::from("service response"))),
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {}

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

async fn handle_request(
    req: Request<Body>,
    limiter: Arc<RateLimiter>,
) -> Result<Response<Body>, hyper::Error> {
    let remote_addr = req.remote_addr().expect("Remote address missing");

    if !limiter.allow(remote_addr) {
        return Ok(Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .body(Body::from("Too many requests"))
            .unwrap());
    }

    println!(
        "Received request from {}:{}",
        remote_addr.ip(),
        remote_addr.port()
    );
    let response = service_handler(req.uri().path()).await;
    response
}

#[tokio::main]
async fn main() {
    let limiter = Arc::new(RateLimiter::new());

    let make_srv = make_service_fn(move |_conn| {
        let limiter = Arc::clone(&limiter);
        let service = service_fn(move |req| handle_request(req, Arc::clone(&limiter)));
        async { Ok::<_, hyper::Error>(service) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_srv);
    println!("Ocypode Gateway listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

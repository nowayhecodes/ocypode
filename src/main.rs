use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

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
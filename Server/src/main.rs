use priority_queue::PriorityQueue;
use rust::models::Client;
use rust::server::start_server;
use rust::usage_monitor::start_usage_monitor;
use std::sync::Arc;

use std::error::Error;
use std::sync::Mutex;

fn main() -> Result<(), Box<dyn Error>> {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let clients_priority_queue: Arc<Mutex<PriorityQueue<Client, u32>>> =
        Arc::new(Mutex::new(PriorityQueue::new()));
    pool.install(|| {
        start_usage_monitor();
        start_server(clients_priority_queue);
    });
    return Ok(());
}

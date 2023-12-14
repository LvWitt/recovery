use priority_queue::PriorityQueue;
use rust::files_generator::clear_usage_report;
use rust::models::Client;
use rust::server::start_server;
use std::sync::Arc;

use std::error::Error;
use std::sync::Mutex;

fn main() -> Result<(), Box<dyn Error>> {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let clients_priority_queue: Arc<Mutex<PriorityQueue<Client, u32>>> =
        Arc::new(Mutex::new(PriorityQueue::new()));
    pool.install(|| {
        clear_usage_report();
        start_server(clients_priority_queue);
    });
    return Ok(());
}

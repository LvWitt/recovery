use std::{
    io::{ErrorKind, Read},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
    thread,
};
use priority_queue::PriorityQueue;
use bincode::deserialize;
use std::{error::Error, time::Instant};
use std::sync::Arc;
use rust::models::{Person, Client};
//use rust::csv_handlers;


//const tolerance: f64 = 1e-8;
const LOCAL: &str = "0.0.0.0:8181";
const MSG_SIZE: usize = 200;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}


fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");
    let start1 = Instant::now();
  //  let matrix_h = Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-1.csv", 50816, 3600).unwrap());
   // let matrix_h2 =  Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-2.csv", 27904, 900).unwrap());
    let end1 = Instant::now();
    println!("Leu arquivos 2 matriz em {:?}", end1 - start1);
    let clients_priority_queue:Arc<Mutex<PriorityQueue<Client,u32>>> = Arc::new(Mutex::new(PriorityQueue::new()));
    let (tx, rx) = mpsc::channel::<String>();

    let pq_ref = clients_priority_queue.clone();
    thread::spawn(move || {
        loop {
            if let Some((client, _)) = pq_ref.lock().unwrap().peek() {
                expensive_function(client);
            }
            if let Ok(msg) = rx.try_recv() {
               /*  clients = clients
                .into_iter()
                .filter_map(|mut client:TcpStream| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();*/
            }
            sleep();
        }
    });
    loop {
        if let Ok((socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let _ = tx.clone();
            //clients.push(socket.try_clone().expect("failed to clone client"));
            let pq_ref = clients_priority_queue.clone();
            thread::spawn(move || handle_client(socket, addr.to_string(),  pq_ref));

        }
        sleep();
    }
}

fn expensive_function(_: &Client) -> () {
    todo!()
}


fn handle_client(mut socket: TcpStream, addr: String, clients_priority_queue:   Arc<Mutex<PriorityQueue<Client, u32>>>) {
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];

        match socket.read_exact(&mut buff) {
            Ok(_) => {
                let deserialized_msg: Person = deserialize(&buff).unwrap();
                println!("{:?}", deserialized_msg);
                let mut pq = clients_priority_queue.lock().unwrap();
                let client_data:Client = Client { person: deserialized_msg, tcp_stream: socket.try_clone().unwrap() };
                pq.push(client_data, 0);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Closing connection with: {}", addr);
                break;
            }
        }
        sleep();
    });
}
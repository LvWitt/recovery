use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use bincode::{deserialize, serialize};
use nalgebra::DVector;
use nalgebra_sparse::CsrMatrix;
use priority_queue::PriorityQueue;
use uuid::Uuid;

use crate::{
    algorithms::process_algorithm,
    csv_handlers,
    files_generator::{create_img, create_json_file},
    models::{AlgorithmsReturnType, ChannelMessage, Client, JSONFileData, Response},
};

const LOCAL: &str = "0.0.0.0:8181";
const MSG_SIZE: usize = 600000;
const TOLERANCE: f64 = 1e-4;


pub fn start_server(clients_priority_queue: Arc<Mutex<PriorityQueue<Client, u32>>>) {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");
    let (tx, rx): (Sender<ChannelMessage>, Receiver<ChannelMessage>) = mpsc::channel();

    start_processing_client_message(rx, clients_priority_queue.clone());
    start_processing_algorithm_queue(clients_priority_queue);
    loop {
        if let Ok((socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();
            thread::spawn(move || handle_client(socket, addr.to_string(), tx));
        }
        sleep(100);
    }
}

fn handle_client(mut socket: TcpStream, addr: String, tx: Sender<ChannelMessage>) {
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];

        match socket.read_exact(&mut buff) {
            Ok(_) => {
                let deserialized_msg: Response = deserialize(&buff).unwrap();
                //println!("{:?}", deserialized_msg.tipo_algoritmo);
                let client_data: Client = Client {
                    request: deserialized_msg,
                    client_id: Uuid::new_v4(),
                    tcp_stream: Arc::new(socket.try_clone().unwrap()),
                };
                tx.send(ChannelMessage::ClientData(client_data))
                    .expect("failed to send message to rx");
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Closing connection with: {}", addr);
                break;
            }
        }
        sleep(100);
    });
}

fn start_processing_client_message(
    rx: mpsc::Receiver<ChannelMessage>,
    clients_priority_queue: Arc<Mutex<PriorityQueue<Client, u32>>>,
) {
    rayon::spawn(move || loop {
        if let Ok(msg) = rx.try_recv() {
            if let ChannelMessage::ClientData(client_data) = msg {
                let mut pq = clients_priority_queue.lock().unwrap();
                let mut priority_number = 1;
                //aqui é onde vai a prioridade
                if client_data.request.tamanho == 30 {
                    priority_number = 2;
                }
                pq.push(client_data.clone(), priority_number as u32);

                let mut p: Vec<u8> = serialize("Adicionado a fila de prioridades").unwrap();
                p.resize(MSG_SIZE, 0);
                let mut tcp_stream: TcpStream = client_data.tcp_stream.clone().try_clone().unwrap();
                let _ = tcp_stream
                    .write_all(&p)
                    .map_err(|_| thread::sleep(Duration::from_millis(50)));
            }
        }
        sleep(100);
    });
}

pub fn start_processing_algorithm_queue(cloned_client_pq: Arc<Mutex<PriorityQueue<Client, u32>>>) {
    let start1 = Instant::now();
    let matrix_h =
        Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-1.csv", 50816, 3600).unwrap());
    let matrix_h2 =
        Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-2.csv", 27904, 900).unwrap());
    let end1 = Instant::now();
    println!("Leu 2 arquivos em {:?}", end1 - start1);
    rayon::spawn(move || loop {
        let client_data = {
            let mut pq = cloned_client_pq.lock().unwrap();
            pq.pop()
        };
        if let Some((client, priority)) = client_data {
            let mh = matrix_h.clone();
            let mh2= matrix_h2.clone();
            rayon::spawn(move || {
                let d_vector = DVector::from_vec(client.request.sinal);

                let result = match (client.request.tipo_algoritmo, client.request.tamanho) {
                    (1, 60) => process_algorithm(&mh, &d_vector, TOLERANCE, "cgne"),
                    (2, 60) => process_algorithm(&mh, &d_vector, TOLERANCE, "cgnr"),
                    (1, 30) => process_algorithm(&mh2, &d_vector, TOLERANCE, "cgne"),
                    (2, 30) => process_algorithm(&mh2, &d_vector, TOLERANCE, "cgnr"),
                    _ => {
                        println!(
                            "Unsupported algorithm type: {}",
                            client.request.tipo_algoritmo
                        );
                        return;
                    }
                };
                match result {
                    Ok(data) => match data {
                        AlgorithmsReturnType::CGNEReturnType(algorithm_data) => {
                            create_img(
                                algorithm_data.image_vector,
                                client.request.tamanho.try_into().unwrap(),
                                client.client_id,
                            );
                            create_json_file(
                                JSONFileData {
                                    iterations: algorithm_data.iterations,
                                    reconstruction_time: algorithm_data.reconstruction_time,
                                    reconstruction_start_time: algorithm_data
                                        .reconstruction_start_time
                                        .naive_local(),
                                    reconstruction_end_time: algorithm_data
                                        .reconstruction_end_time
                                        .naive_local(),
                                    image_size: client.request.tamanho as u32,
                                    algorithm: algorithm_data.alghorithm,
                                    client_id: client.client_id,
                                },
                                client.client_id,
                            );

                            println!(
                                "Terminou processamento do cliente: {:?}, Priority: {}, execucao:{:?}",
                                client.client_id, priority, algorithm_data.reconstruction_time
                            );
                        }
                        AlgorithmsReturnType::CGNRReturnType(algorithm_data) => {
                            create_img(
                                algorithm_data.image_vector,
                                client.request.tamanho.try_into().unwrap(),
                                client.client_id,
                            );
                            create_json_file(
                                JSONFileData {
                                    iterations: algorithm_data.iterations,
                                    reconstruction_time: algorithm_data.reconstruction_time,
                                    reconstruction_start_time: algorithm_data
                                        .reconstruction_start_time
                                        .naive_local(),
                                    reconstruction_end_time: algorithm_data
                                        .reconstruction_end_time
                                        .naive_local(),
                                    image_size: client.request.tamanho as u32,
                                    algorithm: algorithm_data.alghorithm,
                                    client_id: client.client_id,
                                },
                                client.client_id,
                            );

                            println!(
                                "Terminou processamento do cliente: {:?}, Priority: {}, execucao:{:?}",
                                client.client_id, priority, algorithm_data.reconstruction_time
                            );
                        }
                    },
                    Err(err) => {
                        println!("Error processing algorithm: {}", err);
                    }
                }
            });
        } else {
            // Fila de prioridade vazia, espere um pouco antes de verificar novamente
            sleep(100);
        }
    });
}

fn sleep(milis: u64) {
    thread::sleep(::std::time::Duration::from_millis(milis));
}
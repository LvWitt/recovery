use bincode::{deserialize, serialize};
use nalgebra::DVector;
use nalgebra_sparse::CsrMatrix;
use priority_queue::PriorityQueue;
use rust::algorithms::{cgne, cgnr};
use rust::csv_handlers;
use rust::files_generator::{create_img, create_json_file, ImageSize};
use rust::models::{AlgorithmsReturnType, ChannelMessage, Client, JSONFileData, Response};
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, time::Instant};
use std::{
    io::{ErrorKind, Read},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread,
};
use uuid::Uuid;
const LOCAL: &str = "0.0.0.0:8181";
const MSG_SIZE: usize = 600000;
const TOLERANCE: f64 = 1e-4;

fn sleep(milis: u64) {
    thread::sleep(::std::time::Duration::from_millis(milis));
}

fn main() -> Result<(), Box<dyn Error>> {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();

    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");
    let start1 = Instant::now();
    let matrix_h =
        Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-1.csv", 50816, 3600).unwrap());
    let _ =
        Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-2.csv", 27904, 900).unwrap());

    /*let vector_teste =
        Arc::new(csv_handlers::create_vector_from_csv("./src/Data/GRANDE-1.csv").unwrap());*/

    let end1 = Instant::now();

    println!("Leu arquivos 2 matriz em {:?}", end1 - start1);
    let clients_priority_queue: Arc<Mutex<PriorityQueue<Client, u32>>> =
        Arc::new(Mutex::new(PriorityQueue::new()));
    let (tx, rx): (Sender<ChannelMessage>, Receiver<ChannelMessage>) = mpsc::channel();

    let cloned_client_pq = clients_priority_queue.clone();
    pool.install(|| {
        rayon::spawn(move || loop {
            if let Ok(msg) = rx.try_recv() {
                if let ChannelMessage::ClientData(client_data) = msg {
                    let mut pq = clients_priority_queue.lock().unwrap();
                    let priority_number = 1;
                    //aqui é onde vai a prioridade

                    pq.push(client_data.clone(), priority_number as u32);

                    let mut p: Vec<u8> = serialize("Adicionado a fila de prioridades").unwrap();
                    p.resize(MSG_SIZE, 0);
                    let mut tcp_stream: TcpStream =
                        client_data.tcp_stream.clone().try_clone().unwrap();
                    let _ = tcp_stream
                        .write_all(&p)
                        .map_err(|_| thread::sleep(Duration::from_millis(50)));
                }
            }
            sleep(100);
        });
        //thread que processa essa merda
        rayon::spawn(move || loop {
            let client_data = {
                let mut pq = cloned_client_pq.lock().unwrap();
                pq.pop()
            };
            let mh = matrix_h.clone();
            if let Some((client, priority)) = client_data {
                rayon::spawn(move ||  {
                    let d_vector = DVector::from_vec(client.request.sinal);
                    let result = match client.request.tipo_algoritmo {
                        1 => process_algorithm( &mh, &d_vector, TOLERANCE, "cgne"),
                        2 => process_algorithm( &mh, &d_vector, TOLERANCE, "cgnr"),
                        _ => {
                            println!("Unsupported algorithm type: {}", client.request.tipo_algoritmo);
                            return;
                        }
                    };
                    match result {
                        Ok(data) => {
                            match data {
                                AlgorithmsReturnType::CGNEReturnType(alghorithm_data) =>{
                                    create_img(alghorithm_data.image_vector, ImageSize::Medium, client.client_id);
                                    create_json_file(
                                        JSONFileData {
                                            iterations: alghorithm_data.iterations,
                                            reconstruction_time: alghorithm_data.reconstruction_time,
                                            reconstruction_start_time: alghorithm_data.reconstruction_start_time.naive_local(),
                                            reconstruction_end_time: alghorithm_data.reconstruction_end_time.naive_local(),
                                            image_size: client.request.tamanho as u32,
                                            alghorithm: alghorithm_data.alghorithm,
                                            client_id: client.client_id,
                                        },
                                        client.client_id,
                                    );
                
                                    println!(
                                        "Terminou processamento do cliente: {:?}, Priority: {}, execucao:{:?}",
                                        client.client_id, priority, alghorithm_data.reconstruction_time
                                    ); 
                                }
                                AlgorithmsReturnType::CGNRReturnType(alghorithm_data) => {
                                    create_img(alghorithm_data.image_vector, ImageSize::Medium, client.client_id);
                                    create_json_file(
                                        JSONFileData {
                                            iterations: alghorithm_data.iterations,
                                            reconstruction_time: alghorithm_data.reconstruction_time,
                                            reconstruction_start_time: alghorithm_data.reconstruction_start_time.naive_local(),
                                            reconstruction_end_time: alghorithm_data.reconstruction_end_time.naive_local(),
                                            image_size: client.request.tamanho as u32,
                                            alghorithm: alghorithm_data.alghorithm,
                                            client_id: client.client_id,
                                        },
                                        client.client_id,
                                    );
                
                                    println!(
                                        "Terminou processamento do cliente: {:?}, Priority: {}, execucao:{:?}",
                                        client.client_id, priority, alghorithm_data.reconstruction_time
                                    ); 
                                },
                            }
                       
                        }
                        Err(err) => {
                            println!("Error processing algorithm: {}", err);
                        }
                    }
                })
            } else {
                // Fila de prioridade vazia, espere um pouco antes de verificar novamente
                sleep(100);
            }
        });
    });

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
                // println!("{:?}", deserialized_msg);
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
fn process_algorithm(
    mh: &CsrMatrix<f64>,
    vt: &DVector<f64>,
    tolerance: f64,
    algorithm_name: &str,
) -> Result<AlgorithmsReturnType, String> {
    let algorithm_data:AlgorithmsReturnType = match algorithm_name {
        "cgne" => rust::models::AlgorithmsReturnType::CGNEReturnType(cgne(mh, vt, tolerance)),
        "cgnr" => rust::models::AlgorithmsReturnType::CGNRReturnType(cgnr(mh, vt, tolerance)),
        _ => return Err(format!("Unsupported algorithm type: {}", algorithm_name)), 
    };
    println!("Processed");
   return Ok(algorithm_data)
}

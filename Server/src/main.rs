use bincode::{deserialize, serialize};
use chrono::Local;
use nalgebra::DVector;
use nalgebra_sparse::CsrMatrix;
use priority_queue::PriorityQueue;

use rust::algorithms:: process_algorithm;
use rust::csv_handlers;
use rust::files_generator::{create_img, create_json_file, creat_usage_report};
use rust::models::{AlgorithmsReturnType, ChannelMessage, Client, JSONFileData, Response, UsageReport};
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
use sysinfo::{CpuExt, System, SystemExt};
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
    let matrix_h2 =
        Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-2.csv", 27904, 900).unwrap());
    let report_start =  Local::now();
    let mut cpu_usage:Vec<f32> = vec![];
    let mut ram_usage:Vec<u64> = vec![];

    let end1 = Instant::now();
    println!("Leu arquivos 2 matriz em {:?}", end1 - start1);

    let clients_priority_queue: Arc<Mutex<PriorityQueue<Client, u32>>> =
        Arc::new(Mutex::new(PriorityQueue::new()));
    let (tx, rx): (Sender<ChannelMessage>, Receiver<ChannelMessage>) = mpsc::channel();
    let mut sys = System::new();
    let cloned_client_pq = clients_priority_queue.clone(); 
    pool.install(|| {
        rayon::spawn(move || loop {
            if let Ok(msg) = rx.try_recv() {
                if let ChannelMessage::ClientData(client_data) = msg {
                    let mut pq = clients_priority_queue.lock().unwrap();
                    let mut priority_number = 1;
                    //aqui é onde vai a prioridade
                    if client_data.request.tamanho==30 {
                        priority_number = 2;
                    }
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
            let mh2: Arc<CsrMatrix<f64>> = matrix_h2.clone();
            if let Some((client, priority)) = client_data {
                rayon::spawn(move ||  {
                    let d_vector = DVector::from_vec(client.request.sinal);
               
                    let result = match (client.request.tipo_algoritmo, client.request.tamanho) {
                        (1,60) => process_algorithm( &mh, &d_vector, TOLERANCE, "cgne"),
                        (2,60) => process_algorithm( &mh, &d_vector, TOLERANCE, "cgnr"),
                        (1,30) => process_algorithm( &mh2, &d_vector, TOLERANCE, "cgne"),
                        (2,30) => process_algorithm( &mh2, &d_vector, TOLERANCE, "cgnr"),
                        _ => {
                            println!("Unsupported algorithm type: {}", client.request.tipo_algoritmo);
                            return;
                        }
                    };
                    match result {
                        Ok(data) => {
                            match data {
                                AlgorithmsReturnType::CGNEReturnType(alghorithm_data) =>{
                                    create_img(alghorithm_data.image_vector, client.request.tamanho.try_into().unwrap(), client.client_id);
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
                                    create_img(alghorithm_data.image_vector, client.request.tamanho.try_into().unwrap(), client.client_id);
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
        //thread que monitora a desgraça do consumo
        rayon::spawn(move || loop{
                sys.refresh_cpu();
                sys.refresh_memory();
                cpu_usage.push(sys.global_cpu_info().cpu_usage());
                ram_usage.push(sys.used_memory());  
                let report_end =  Local::now();
                creat_usage_report(UsageReport{
                start_time:report_start.naive_local(),
                end_time: report_end.naive_local(),
                cpu_usage:cpu_usage.clone(),
                ram_usage:ram_usage.clone()
    });
                std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
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


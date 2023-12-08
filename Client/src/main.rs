use std::net::TcpStream;
use std::time::Duration;

use std::{
    io::{ErrorKind, Read, Write},
    sync::mpsc,
    thread,
};

use rand::Rng;

use bincode::{serialize, deserialize};
use mpsc::TryRecvError;
use serde::{Deserialize, Serialize};

pub mod algorithms;
pub mod readers;
use crate::{algorithms::apply_gain_signal, readers::create_vector_from_csv};

const LOCAL: &str = "127.0.0.1:8181";
const MSG_SIZE: usize = 600000;


#[derive(Serialize, Deserialize, Debug)]
struct Request {
    tipo_algoritmo: i32,
    tipo_sinal: i32,
    tipo_matriz: i32,
    tamanho: i32,
    sinal: Vec<f64>,
}

fn main() {
    //create_Randon_Request();
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<u32>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg:String = deserialize(&buff).expect("invalid utf8 message");
                println!("message recv {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut rng = rand::thread_rng();
                let number_requests = rng.gen_range(0..10);

                for _ in 0..number_requests {
                    let request = create_process_request();
                    let mut p: Vec<u8> = serialize(&request).unwrap();

                    p.resize(MSG_SIZE, 0);
                    match client.write_all(&p) {
                        Ok(_) =>  println!("Request sent {:?}", msg),
                        Err(_) =>  thread::sleep(Duration::from_millis(50)),
                    }
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message:");
    loop {
       for i in 0..100 {
            let _ = tx.clone().send(i);
            thread::sleep(Duration::from_millis(500));

        }

    }
}

fn create_process_request() -> Request {
    let mut rng = rand::thread_rng();

    let size = rng.gen_range(30..=60);
    let matriz = rng.gen_range(1..2);
    let signal;

    if matriz == 30 {
        signal = rng.gen_range(1..3);
    } else {
        signal = rng.gen_range(3..5);
    }

    let algorithm = rng.gen_range(1..2);

    let mut vector = create_vector_from_csv("./Data/G-60-1.csv").unwrap();
    vector = apply_gain_signal(vector);
    let aux: Vec<f64> = vector.data.as_vec().to_vec();

    let info = Request {
        tipo_algoritmo: algorithm,
        tipo_sinal: signal,
        tipo_matriz: matriz,
        tamanho: size,
        sinal: aux,
    };

    //print!("{:?}", info);
    return info;
}

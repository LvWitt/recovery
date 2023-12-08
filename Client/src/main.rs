use std::io;
use std::net::TcpStream;
use std::time::Duration;

use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

use bincode::serialize;
use mpsc::TryRecvError;
use serde::{Deserialize, Serialize};

use std::{error::Error, time::Instant};
pub mod algorithms;
pub mod readers;
use crate::{algorithms::applyGainSignal, readers::create_vector_from_csv};
use nalgebra::DVector;

const tolerance: f64 = 1e-8;

const LOCAL: &str = "127.0.0.1:8181";
const MSG_SIZE: usize = 200;

#[derive(Serialize, Deserialize)]
struct Person {
    id: u32,
}
fn main() {
    createProcess();
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<u32>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = String::from_utf8(buff).expect("invalid utf8 message");
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
                let person1 = Person { id: msg };

                let mut p: Vec<u8> = serialize(&person1).unwrap();

                p.resize(MSG_SIZE, 0);
                client.write_all(&p).expect("writing to socket failed");

                println!("message sent {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message:");
loop{
    for i in 0..100 {
        thread::sleep(Duration::from_millis(500));
        let _ = tx.send(i).is_err();
    }
}
    
}

fn createProcess() {
    //  let mut vector = create_vector_from_csv("./Data/G-30.csv").unwrap();
    //vector = applyGainSignal(vector);
}

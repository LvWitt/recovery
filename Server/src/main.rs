use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

use bincode::deserialize;
use std::{error::Error, time::Instant};

use rust::algorithms::{cgne, cgnr};
use rust::csv_handlers;
use rust::image_generator::create_img;

use serde::Deserialize;
const tolerance: f64 = 1e-8;
use std::sync::Arc;
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
    let matrix_h = Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-1.csv", 50816, 3600).unwrap());
    let matrix_h2 =  Arc::new(csv_handlers::create_matrix_from_csv("./src/Data/H-2.csv", 27904, 900).unwrap());
    let end1 = Instant::now();
    println!("Leu arquivos 2 matriz em {:?}", end1 - start1);
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            let mut num = 0;
           /*  for _ in 0..2 {
                num += 1;
                thread::spawn(move || {
                    println!("----COMECANDO {}----", num);
                    let start1 = Instant::now();
                    let vector =
                        csv_handlers::create_vector_from_csv("./src/Data/G-1.csv").unwrap();
                    let matrix =
                        csv_handlers::create_matrix_from_csv("./src/Data/H-1.csv", 50816, 3600)
                            .unwrap();
                    // let matrix = create_matrix_from_csv("../Data/H-2.csv",27904,900).unwrap();
                    let end1 = Instant::now();
                    println!("{} - Leu arquivos em {:?}", num, end1 - start1);

                    // println!("matriz{:?}",matrix.row(0).get_entry(60));
                    let start = Instant::now();
                    //let cgne = cgne(matrix, vector, tolerance);
                    //println!("{:?}", cgne.get(0));
                    let cgnr = cgnr(matrix, vector, tolerance);
                    //  println!("{:?}", cgne);
                    let end = Instant::now();
                    println!("{} - algoritmo em {:?}", num, end - start);
                    println!("----TERMINOU {}----", num);
                    //println!("{:?}",cgne);
                    
                    create_img(cgnr, 60);
                    //create_img(cgnr, 30);
                });
            }*/
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        println!("{:?}", buff);
                        //let teste: Person = deserialize(&buff).unwrap();
                        //println!("{:?}", teste)
                        //let msg = String::from_utf8(msg).expect("invalid utf8 message");
                        //let id = u32::from_be_bytes(msg[0..4].try_into().unwrap());
                        // println!("{} {:?}", addr, teste);
                        //  tx.send(id).expect("failed to send message to rx");
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

        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}

use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

use std::{error::Error, time::Instant};
pub mod algorithms;
pub mod readers;
use crate::{
    algorithms::{cgne, cgnr},
    readers::{create_matrix_from_csv, create_vector_from_csv},
};
use bincode::deserialize;
use nalgebra:: DVector;
use serde::{Serialize, Deserialize};

const tolerance: f64 = 1e-8;

const LOCAL: &str = "0.0.0.0:8181";
const MSG_SIZE: usize = 200;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() -> Result<(), Box<dyn Error>> {


    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            
            let mut num = 0;
            for _ in 0..2 {
                num += 1;
             /*    thread::spawn( move || {
                        println!("----COMECANDO {}----", num);
                        let start1 = Instant::now();
                        let vector = create_vector_from_csv("./src/Data/G-1.csv").unwrap();
                        let matrix = create_matrix_from_csv("./src/Data/H-1.csv", 50816, 3600).unwrap();
                    // let matrix = create_matrix_from_csv("../Data/H-2.csv",27904,900).unwrap();
                        let end1 = Instant::now();
                        println!("{} - Leu arquivos em {:?}", num, end1 - start1);
                    
                        // println!("matriz{:?}",matrix.row(0).get_entry(60));
                        let start = Instant::now();
                    //let cgne = cgne(matrix, vector, tolerance);
                        //println!("{:?}", cgne.get(0));
                        let cgnr = cgnr(matrix,vector,tolerance);
                        //  println!("{:?}", cgne);
                        let end = Instant::now();
                        println!("{} - algoritmo em {:?}", num, end - start);
                        println!("----TERMINOU {}----", num);
                        //println!("{:?}",cgne);
                        create_img(cgnr, 60);
                        //create_img(cgnr, 30);
                    }
                );*/
            }

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
           
                        println!("{:?}",buff);
                        let teste: Person = deserialize(&buff).unwrap();
                        println!("{:?}", teste)
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

fn create_img(vector: DVector<f64>, size:u32) {
    let imgx = size;
    let imgy = size;

    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(imgx, imgy);

    for i in 0..imgx {
        for j in 0..imgy {
            let img_pixel = imgbuf.get_pixel_mut(i, j);
            let index: usize = ((i * imgx) + j) as usize;
            let pixel_value = vector.get(index).unwrap();
            let r = (pixel_value * 255.0) as u8;
            *img_pixel = image::Rgb([r, 0, 0]);
        }
    }

    let _ = imgbuf.save("MEUFILHO.png");
}

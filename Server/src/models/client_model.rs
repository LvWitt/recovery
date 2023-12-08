use std::{hash::{Hash, Hasher}, net::TcpStream, sync::Arc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(PartialEq, Deserialize,Debug, Clone)]
pub struct Response{
    pub tipo_algoritmo:i32,
    pub tipo_sinal:i32,
    pub tipo_matriz : i32,
    pub tamanho:i32,
    pub sinal:Vec<f64>
}
#[derive(Debug,Clone)]
pub struct Client{
    pub request:Response,
    pub client_id:Uuid ,
    pub tcp_stream:Arc<TcpStream>
}

impl Hash for Client {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.client_id.hash(state);
    }   
}
impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.client_id == other.client_id
    }
}
impl Eq for Client {}


pub enum ChannelMessage {
    ClientData(Client),
    ConfirmMessage(String)
}
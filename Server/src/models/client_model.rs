use std::{hash::{Hash, Hasher}, net::TcpStream, sync::Arc};
use serde::Deserialize;

#[derive(PartialEq, Eq, Hash, Deserialize,Debug, Clone)]
pub struct Person{
    pub id:u32
}
#[derive(Debug,Clone)]
pub struct Client{
    pub person:Person,
    pub tcp_stream:Arc<TcpStream>
}

impl Hash for Client {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.person.hash(state);
    }   
}
impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.person == other.person
    }
}
impl Eq for Client {}


pub enum ChannelMessage {
    ClientData(Client),
    ConfirmMessage(String)
}
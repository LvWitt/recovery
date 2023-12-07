use std::{hash::{Hash, Hasher}, net::TcpStream};
use serde::Deserialize;

#[derive(PartialEq, Eq, Hash, Deserialize,Debug)]
pub struct Person{
    id:u32
}
pub struct Client{
    pub person:Person,
    pub tcp_stream:TcpStream
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

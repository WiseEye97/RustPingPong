use std::net::{TcpListener, TcpStream,Shutdown};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::str;



pub struct Connector{
    address : String,
    connection : Option<TcpStream>,
    onMsg : mpsc::Sender<String>,
}

pub struct Wrapper{
    inner: Arc<Mutex<Connector>>
}



impl Connector{
    pub fn new(address : String,onMsg : mpsc::Sender<String>) -> Connector{
        Connector {address,connection : None,onMsg}
    }

    pub fn connect(&mut self){
        let add = self.address.as_str();
        let con = TcpStream::connect(add).unwrap();
        self.connection = Some(con);
    }

    pub fn listen(&mut self){
        
        let mut buffer = String::new();
       
        let mut ptr = self.connection.as_ref().unwrap();

        loop {
            let mut data = [0 as u8; 50];

            match ptr.read(&mut data){
                Ok(_s) => {
                        let st =  str::from_utf8(&data).unwrap();
                        self.onMsg.send(String::from(st));
                    }
                _ => (),
            }

        }
       
    }
}

impl Wrapper{
    pub fn new(address : String,onMsg : mpsc::Sender<String>) -> Wrapper{
        Wrapper {inner : Arc::new(Mutex::new(Connector::new(address,onMsg)))} 
    }
    pub fn start(&mut self){
        let mut local_self = self.inner.clone();

        thread::spawn(move || {
            let l = local_self.lock().unwrap().listen();
        });
    }
}
use std::net::{TcpListener, TcpStream,Shutdown};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::str;



pub struct Connector{
    address : String,
    connection : Option<TcpStream>,
    on_msg : mpsc::Sender<String>,
}

pub struct Wrapper{
    inner: Arc<Mutex<Connector>>
}



impl Connector{
    pub fn new(address : String,on_msg : mpsc::Sender<String>) -> Connector{
        Connector {address,connection : None,on_msg}
    }

    pub fn connect(&mut self){
        let add = self.address.as_str();

        if let Ok(con) = TcpStream::connect(add) {
            self.connection = Some(con);
        }else{
            println!("Cant connect");
        }
        
    }

    pub fn listen(&mut self){
        
        let mut buffer = String::new();
       
        let mut ptr = self.connection.as_ref().unwrap();

        loop {
            let mut data = [0 as u8; 50];

            match ptr.read(&mut data){
                Ok(_s) => {
                        let st =  str::from_utf8(&data).unwrap();
                        if let Ok(_) = self.on_msg.send(String::from(st)){

                        }
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
        let local_self = self.inner.clone();

        thread::spawn(move || {
            local_self.lock().unwrap().connect();
            local_self.lock().unwrap().listen();
        });
    }
}
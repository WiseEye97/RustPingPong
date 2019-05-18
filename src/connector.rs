use std::net::{TcpListener, TcpStream,Shutdown};
use std::io::{Read, Write};

use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::str;



pub struct Connector{
    connection : TcpStream,
    on_msg : mpsc::Sender<String>,
}

pub struct Wrapper{
    inner: Arc<Mutex<Connector>>
}



impl Connector{
    pub fn new(stream : TcpStream,on_msg : mpsc::Sender<String>) -> Connector{
        Connector {connection : stream,on_msg}
    }

    pub fn listen(&mut self){
        
        let mut buffer = String::new();
       
        let mut ptr = &self.connection;

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
    pub fn new(stream : TcpStream,onMsg : mpsc::Sender<String>) -> Wrapper{
        Wrapper {inner : Arc::new(Mutex::new(Connector::new(stream,onMsg)))} 
    }
    pub fn start(&mut self){
        let local_self = self.inner.clone();

        thread::spawn(move || {
            if let Ok(mut x) = local_self.lock(){
                println!("locked 2");
                x.listen();
            }
        });
    }
}
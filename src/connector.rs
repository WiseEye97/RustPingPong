use std::net::{TcpListener, TcpStream,Shutdown};
use std::io::{Read, Write};

use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::str;



pub struct Connector{
    address : String,
    connection : Option<TcpStream>,
    connection_sender : Option<TcpStream>,
    on_msg : mpsc::Sender<String>,
    on_send : mpsc::Receiver<String>,
}

pub struct Wrapper{
    inner: Arc<Mutex<Connector>>
}



impl Connector{
    pub fn new(address : String,on_msg : mpsc::Sender<String>,on_send : mpsc::Receiver<String>) -> Connector{
        Connector {address,connection_sender : None,connection : None,on_msg,on_send}
    }

    pub fn connect(&mut self){
        let add = self.address.as_str();

        if let Ok(con) = TcpStream::connect(add) {

            let mut tcp = con.try_clone().unwrap();

            self.connection = Some(con);
            self.connection_sender = Some(tcp);

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

    pub fn sender(&mut self){

        let mut ptr = self.connection_sender.as_ref().unwrap();

        loop{
            match self.on_send.try_recv(){
                 Ok(message) => {
                    let bytes = message.as_bytes();
                    ptr.write(bytes).unwrap();       
                 },
                 _ => ()
            }
        }
    }
}

impl Wrapper{
    pub fn new(address : String,onMsg : mpsc::Sender<String>,on_send : mpsc::Receiver<String>) -> Wrapper{
        Wrapper {inner : Arc::new(Mutex::new(Connector::new(address,onMsg,on_send)))} 
    }
    pub fn start(&mut self){
        let local_self = self.inner.clone();

        thread::spawn(move || {
            local_self.lock().unwrap().connect();
            local_self.lock().unwrap().listen();
        });
    }
    
    pub fn start_sender(&mut self){
        let local_self = self.inner.clone();

        thread::spawn(move || {
            local_self.lock().unwrap().sender();
        });
    }
}
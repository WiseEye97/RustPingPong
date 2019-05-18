use std::net::{TcpListener, TcpStream,Shutdown};
use std::io::{Read, Write};

use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::str;



pub struct ErlangSender{
    connection : TcpStream,
    on_send : mpsc::Receiver<String>,
}

pub struct WrapperSender{
    inner: Arc<Mutex<ErlangSender>>
}



impl ErlangSender{
    pub fn new(stream : TcpStream,on_send : mpsc::Receiver<String>) -> ErlangSender{
        ErlangSender {connection : stream,on_send}
    }

   
    pub fn sender(&mut self){

        let mut ptr = &self.connection;

        loop{
            match self.on_send.try_recv(){
                 Ok(message) => {
                    println!("message to send -> {}",message); 
                    let bytes = message.as_bytes();
                    ptr.write(bytes).unwrap();       
                 },
                 _ => {
                     //println!("No message");
                 }
            }
        }
    }
}

impl WrapperSender{
    pub fn new(stream : TcpStream,on_send : mpsc::Receiver<String>) -> WrapperSender{
        WrapperSender {inner : Arc::new(Mutex::new(ErlangSender::new(stream,on_send)))} 
    }
    pub fn start(&mut self){
        let local_self = self.inner.clone();

        thread::spawn(move || {
            if let Ok(mut x) = local_self.lock(){
                println!("locked 2");
                x.sender();
            }
        });
    }
    
}
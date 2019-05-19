use std::net::{ TcpStream};
use std::io::{Read};

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
        
       
        let mut ptr = &self.connection;

        loop {
            let mut data = [0 as u8; 50];

            match ptr.read(&mut data){
                Ok(_s) => {
                        let mut last = 0;

                        for b in data.iter(){
                            if *b == (0 as u8){
                                break;
                            }
                            last += 1;
                        }

                       
                        let st =  str::from_utf8(&data[0..last]).unwrap();

                        println!("Message from server -> {}",st);

                        if let Ok(_) = self.on_msg.send(String::from(st)){
                            
                        }
                    }
                _ => (),
            }

        }
       
    }

}

impl Wrapper{
    pub fn new(stream : TcpStream,on_msg : mpsc::Sender<String>) -> Wrapper{
        Wrapper {inner : Arc::new(Mutex::new(Connector::new(stream,on_msg)))} 
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
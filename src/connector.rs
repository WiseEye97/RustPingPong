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

type ParsedObj = String;

fn json_pre_parser(s : &String) -> (Vec<ParsedObj>,String){

    let mut level = 0;

    let mut current = String::new();
    let mut parsed : Vec<ParsedObj> = Vec::new();

    for c in s.chars(){
        match c{
            '{' => level += 1,
            '}' => level -= 1,
            _ if level == 0 => {
                return (parsed,String::new());        
            },
            _ => (),
        };
        
        current.push(c);
 
        if level == 0{
            parsed.push(current.clone());
            current.clear();
        }        
    }
    
    (parsed,current)
}


impl Connector{
    pub fn new(stream : TcpStream,on_msg : mpsc::Sender<String>) -> Connector{
        Connector {connection : stream,on_msg}
    }

    pub fn listen(&mut self){
        
       
        let mut ptr = &self.connection;

        let mut buffer = String::new();

        loop {
            let mut data = [0 as u8; 50];

            match ptr.read(&mut data){
                Ok(_s) => {

                        let st =  str::from_utf8(&data).unwrap();
                        
                        buffer.push_str(st);

                        let (parsed_jsons,rest) = json_pre_parser(&buffer);

                        buffer = rest;

                        for parsed_json in parsed_jsons.iter(){
                            self.on_msg.send(parsed_json.clone()).unwrap(); 
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
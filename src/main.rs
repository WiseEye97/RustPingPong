extern crate piston_window;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use piston_window::*;
use piston_window::types::Color;
use std::io::{Read,Write,self, BufRead};
use std::sync::mpsc;
use std::net::{TcpStream};

use crate::connector::*;
use crate::types::*;
use crate::sender::*;

mod connector;
mod types;
mod sender;



fn createTcp(address : String,on_msg : mpsc::Sender<String>,on_send : mpsc::Receiver<String>) -> Option<(Wrapper,WrapperSender)> {
    if let Ok(mut con) = TcpStream::connect(address) {
            let mut cloned = con.try_clone().unwrap();
            Some((Wrapper::new(con,on_msg),WrapperSender::new(cloned,on_send)))
            
    }else{
        println!("Cant connect");
        None
    }
}

fn main() {

    let mut tst = TcpMessage::<Pt>::new(String::from("tp"),Pt::new());

    println!("msg -> {}", tst.serialize());

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let (tx,rx) = mpsc::channel::<String>();
    let (tx2,rx2) = mpsc::channel::<String>();

    let (mut wrapper,mut wrapper_sender) = createTcp(String::from("127.0.0.1:7878"),tx,rx2).unwrap();

    wrapper.start();
    wrapper_sender.start();

    println!("Write your username:");

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();


    let mut name_req = NameRequest::new(String::from(line.trim_end()));
    let mut msg = TcpMessage::<NameRequest>::new(String::from("register"),name_req);

    if let Ok(x) = tx2.send(msg.serialize()){
        println!("message send!");
    }
    

    while let Some(event) = window.next(){

        if let Some(Button::Keyboard(key)) = event.press_args() {
           
        }
        
        window.draw_2d(&event, |c, g| {
                
        });

        event.update(|arg| {
              
        });

    }


}

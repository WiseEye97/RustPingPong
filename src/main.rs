extern crate piston_window;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use piston_window::*;
use piston_window::types::Color;
use std::io::{self, BufRead};
use std::sync::{Arc,Mutex};
use std::sync::mpsc;
use std::net::{TcpStream};

use crate::connector::*;
use crate::types::*;
use crate::sender::*;
use crate::game::{Game};
use crate::server_listener::ServerListener;
use crate::drawing::{draw_game};
use crate::config::*;

mod connector;
mod types;
mod sender;
mod game;
mod server_listener;
mod drawing;
mod config;


fn create_tcp(address : String,on_msg : mpsc::Sender<String>,on_send : mpsc::Receiver<String>) -> Option<(Wrapper,WrapperSender)> {
    if let Ok(con) = TcpStream::connect(address) {
            let cloned = con.try_clone().unwrap();
            Some((Wrapper::new(con,on_msg),WrapperSender::new(cloned,on_send)))
            
    }else{
        println!("Cant connect");
        None
    }
}


fn get_vector(key : Key) -> Option<(i32,i32)> {
    match key{
        Key::Left => Some((-MOVE_STEP,0)),
        Key::Right => Some((MOVE_STEP,0)), 
        Key::Up => Some((0,-MOVE_STEP)),
        Key::Down => Some((0,MOVE_STEP)),
        _ => None 
    }
}

fn run_piston(game : Arc<Mutex<Game>>,sender : mpsc::Sender<String>){
    const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

    let mut elapsed = 0.0;

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [1000, 800])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args() {
           if let Some(vctor) = get_vector(key) {
               if let Ok(mut g) = game.lock() {
                   if g.client.move_player(vctor){
                       sender.send(MoveRequest::create_to_json(g.client.pos)).unwrap();
                   }
               }
           }
        }

        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            let mtx = game.lock().unwrap();
            draw_game(&mtx, &c,g);
        });    

        event.update(|arg| {
            elapsed += arg.dt;
            if elapsed > crate::config::UPDATE_TIME {
                game.lock().unwrap().move_ball();
                elapsed = 0.0;
            }
        });
    }
}

fn main() {
    //Main game struct for controlling Game , initially ininitialized with dummy values
    let game_obj : Arc<Mutex<Game>> = Arc::new(Mutex::new(Game::dummy()));

    //Channel for messages from server
    let (tx,rx) = mpsc::channel::<String>();

    //Channel for messages to server
    let (tx2,rx2) = mpsc::channel::<String>();

    //Controller for messagess from server
    let mut srv_listener = ServerListener::new(game_obj.clone(), rx);
    //start listening
    srv_listener.start();

    //connections to server one for sending one for recieving
    let (mut wrapper,mut wrapper_sender) = create_tcp(String::from("127.0.0.1:7878"),tx,rx2).unwrap();

    //start listening
    wrapper.start();
    wrapper_sender.start();


    //get username from stdin
    println!("Write your username:");

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    //Contruct message and send to the server
    let name_req = NameRequest::new(String::from(line.trim_end()));
    let mut msg = TcpMessage::<NameRequest>::new(String::from("register"),name_req);


    tx2.send(msg.serialize()).unwrap();
    
    //Piston Window
    run_piston(game_obj.clone(),tx2);

}

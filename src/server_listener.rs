use crate::game::{Game,Side};
use std::sync::{Arc,Mutex};
use std::sync::mpsc;
use std::thread;
use crate::types::*;

struct InnerListener{
    game : Arc<Mutex<Game>>,
    on_send : mpsc::Receiver<String>,
}

impl InnerListener{
    pub fn new(game : Arc<Mutex<Game>>,on_send : mpsc::Receiver<String>) -> InnerListener{
        InnerListener {game,on_send}
    }
    pub fn listen(&mut self){
        loop{
            match self.on_send.try_recv(){
                Ok(message) => {
                    let encoded = TcpInMessage::get_type(message);
                    match (encoded.tp).as_ref(){
                        "game_init" => {
                            let body = TcpInMessage::get_body::<GameInit>(encoded);
                            if let Ok(mut foo) = self.game.lock() {
                                let (client_side,opp_side) =
                                    match body.side.as_ref(){
                                        "up" => {
                                            (Side::Up,Side::Down)
                                        },
                                        _ => {(Side::Down,Side::Up)}
                                    };
                                println!("Initializing game");    
                                foo.init(client_side,opp_side,500,500,crate::game::Dimensions {width : 50,height : 10});
                            }
                        },
                        "move_req" => {
                            let body = TcpInMessage::get_body::<MoveRequest>(encoded);
                            if let Ok(mut foo) = self.game.lock() {
                                foo.opponent.set_new_pos(body.pos);
                            }else{
                                println!("Cant move");
                            }
                        },
                        _ => {

                        }
                    }
                },
                _ => {
                    
                }
            }
        }
    }
}

pub struct ServerListener{
    inner : Arc<Mutex<InnerListener>>,
}

impl ServerListener{
    pub fn new(game : Arc<Mutex<Game>>,on_send : mpsc::Receiver<String>) -> ServerListener{
        ServerListener {inner : Arc::new(Mutex::new(InnerListener::new(game,on_send)))}
    }
    pub fn start(&mut self){
        let local_self = self.inner.clone();

        thread::spawn(move || {
            if let Ok(mut x) = local_self.lock(){
                x.listen();
            }
        });
    }
}
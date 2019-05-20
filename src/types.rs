use std::*;

use serde_json;


#[derive(Serialize, Deserialize)]
pub struct Pt{
    x : i32,
}

#[derive(Serialize, Deserialize)]
pub struct NameRequest{
    name : String,
}

#[derive(Serialize, Deserialize)]
pub struct MoveRequest{
    pub pos : i32,
}


#[derive(Serialize, Deserialize)]
pub struct GameInit{
    pub side : String,
}

#[derive(Serialize, Deserialize)]
pub struct TcpInMessage{
    pub tp : String,
    body: serde_json::Value,
}

impl TcpInMessage{
    pub fn get_type(message : String) -> TcpInMessage{
        let my_struct: TcpInMessage = serde_json::from_str(&message).unwrap();
        my_struct
    }
    pub fn get_body<T>(mess : TcpInMessage) -> T where for<'de> T: serde::Deserialize<'de> {
        let x: T = serde_json::from_value(mess.body).unwrap();
        x
    } 
}

#[derive(Serialize, Deserialize)]
pub struct TcpMessage<T: serde::Serialize>{
    tp : String,
    content : T,
}

impl <T> TcpMessage<T> where T: serde::Serialize {
    pub fn serialize(&mut self) -> String{
        let res = serde_json::to_string(self).unwrap();
        res
    }
    pub fn new(tp : String,content : T) -> TcpMessage<T>{
        TcpMessage::<T> {tp,content}
    }
    
}


impl NameRequest{
    pub fn new(name : String) -> NameRequest{
        NameRequest {name}
    }
}

impl MoveRequest{
    pub fn create_to_json(pos : i32) -> String{
        let o : TcpMessage<MoveRequest> = TcpMessage::<MoveRequest>::new(String::from("move_req"), MoveRequest {pos});
        serde_json::to_string(&o).unwrap()
    }
}

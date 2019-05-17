extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;

use crate::connector::*;

mod connector;

fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut connector = Connector::new(String::from("127.0.0.1:7878"));
    connector.connect();


    while let Some(event) = window.next(){

        if let Some(Button::Keyboard(key)) = event.press_args() {
           
        }
        
        window.draw_2d(&event, |c, g| {
                
        });

        event.update(|arg| {
              
        });

    }


}

use crate::game::*;
use piston_window::*;
use piston_window::types::Color;

fn draw_player(game_obj : &Game,player : &Player,con: &Context, g: &mut G2d){
    let y = 
        match player.side{
            Side::Down => {
                game_obj.height - player.dim.height - 10
            },
            Side::Up => {
                10
            }
        };
    draw_rectangle([1.0; 4], player.pos, y, player.dim.width, player.dim.height, con, g);
}

fn draw_ball(ball : &Ball,con: &Context, g: &mut G2d){
    let ell = Ellipse::new([1.0,0.0,0.0,1.0]);
    ell.draw([ball.x_pos as f64,ball.y_pos as f64,ball.radius as f64,ball.radius as f64], &con.draw_state, con.transform, g);
}

pub fn draw_game(game_obj : &Game,con: &Context, g: &mut G2d){
   draw_player(game_obj,&game_obj.client, con,g);
   draw_player(game_obj,&game_obj.opponent, con,g); 
   draw_ball(&game_obj.ball, con,g);
}

pub fn draw_rectangle(color: Color,x: i32,y: i32,width: i32,height: i32,con: &Context,g: &mut G2d,) {
    rectangle(
        color,
        [
            x as f64,
            y as f64,
            width as f64,
            height as f64,
        ],
        con.transform,
        g,
    );
}
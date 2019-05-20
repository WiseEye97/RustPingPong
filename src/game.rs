use crate::config::*;


#[derive(Debug)]
pub enum Side{
    Up,
    Down
}

#[derive(Clone, Copy, Debug)]
pub struct Dimensions{
    pub width : i32,
    pub height : i32,
}

pub struct Player{
    pub side : Side,
    pub pos : i32,
    pub dim : Dimensions,
}

#[derive(Debug)]
pub enum MoveResult{
    Moved(Option<(i32,i32)>),
    Lost(Side),
}

impl Player{
    pub fn new(side : Side,pos : i32,dim : Dimensions) -> Player {
        Player {side,pos,dim}
    }
    pub fn move_player(&mut self,(x,_) : (i32,i32)) -> bool{
        let new_pos = self.pos + x;

        if new_pos >= 0 && new_pos <= crate::config::GAME_WIDTH {
            self.pos = new_pos;
            true
        }else{
            false
        }    
    }
    pub fn set_new_pos(&mut self,pos : i32){
        self.pos = pos;
    }
}

pub struct Ball{
    pub x_pos : i32,
    pub y_pos : i32,
    pub vect : (i32,i32),
    pub radius : i32,
}

impl Ball {
    pub fn new(x_pos : i32,y_pos : i32,radius : i32) -> Ball {
        Ball {x_pos,y_pos,vect : (5,5),radius}
    }
}

pub struct Game{
    pub client : Player,
    pub opponent : Player,
    pub ball : Ball,
    pub width : i32,
    pub height : i32,
}

impl Game{
    pub fn init(&mut self,client_side : Side,opponent_side : Side,width : i32,height : i32,players_dim : Dimensions){
        println!("I Am here");
        let half = width / 2;
        
        self.client = Player::new(client_side,half,players_dim);
        self.opponent = Player::new(opponent_side,half,players_dim);
        self.ball = Ball::new(half,height/2,15);
        self.width = width;
        self.height = height;
        
    }
    pub fn dummy() -> Game{
        Game {
            client : Player::new(Side::Up,0, Dimensions {width : 0,height : 0}),
            opponent : Player::new(Side::Up,0,Dimensions {width : 0,height : 0}),
            ball : Ball::new(0,0,0),
            width : 0,
            height : 0
        }
    }

    pub fn detect_collision(&self) -> MoveResult{
        let ball = &self.ball;
        let r = ball.radius;
        let (l,p,u,d) = (ball.x_pos - r,ball.x_pos + r,ball.y_pos - r,ball.y_pos + r);
        let (dx,dy) = ball.vect;

        let (pw,ph) = (self.client.dim.width,self.client.dim.height);

        let (lb,up) = (PLAYER_MARGIN + ph , GAME_HEIGHT - PLAYER_MARGIN - ph);

        if (l < 0 && d > up) || (p > GAME_WIDTH && d > up) || (u < lb && p > GAME_WIDTH) || (u < lb && l < 0){
            println!("bang1");
            return MoveResult::Moved(Some((-dx,-dy)));
        }

        let (upper,lower) = 
            match self.client.side{
                Side::Up => (&self.client,&self.opponent),
                _ => (&self.opponent,&self.client)
            };

        if l < 0 || p > GAME_WIDTH { 
            println!("bang2");
            MoveResult::Moved(Some((-dx,dy)))
        
        }else if u < lb{
            if ball.x_pos > upper.pos && (ball.x_pos < (upper.pos + pw)){
                println!("bang3");
                MoveResult::Moved(Some((dx,-dy)))
            }else{
                MoveResult::Lost(Side::Down)
            }
        }
        else if d > up{
            if ball.x_pos > lower.pos && (ball.x_pos < (lower.pos + pw)){
                println!("bang4");
                MoveResult::Moved(Some((dx,-dy)))
            }else{
                MoveResult::Lost(Side::Up)
            }
        }else{
            MoveResult::Moved(None)
        }
    }

    pub fn move_ball(&mut self){
        let (dx,dy) = self.ball.vect;
        self.ball.x_pos += dx;
        self.ball.y_pos += dy;
        let r = self.detect_collision();
        match r {
            MoveResult::Moved(Some(vc)) => {
                self.ball.vect = vc;
            },
            MoveResult::Lost(_) => {
                self.ball.vect = (0,0);
            },
            _ => {}
        }
    }
}


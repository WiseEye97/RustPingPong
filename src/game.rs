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

impl Player{
    pub fn new(side : Side,pos : i32,dim : Dimensions) -> Player {
        Player {side,pos,dim}
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
        Ball {x_pos,y_pos,vect : (0,5),radius}
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
        let half = width / 2;
        
        self.client = Player::new(client_side,half,players_dim);
        self.opponent = Player::new(opponent_side,half,players_dim);
        self.ball = Ball::new(half,height/2,7);
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
}


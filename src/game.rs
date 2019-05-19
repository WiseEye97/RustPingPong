pub enum Side{
    Up,
    Down
}

pub struct Player{
    side : Side,
    pos : i32,
}

impl Player{
    pub fn new(side : Side,pos : i32) -> Player {
        Player {side,pos}
    }
}

pub struct Ball{
    x_pos : i32,
    y_pos : i32,
    vect : (i32,i32),
}

impl Ball {
    pub fn new(x_pos : i32,y_pos : i32) -> Ball {
        Ball {x_pos,y_pos,vect : (0,5)}
    }
}

pub struct Game{
    client : Player,
    opponent : Player,
    ball : Ball,
    width : i32,
    height : i32,
}

impl Game{
    pub fn init(&mut self,client_side : Side,opponent_side : Side,width : i32,height : i32){
        let half = width / 2;
        
        self.client = Player::new(client_side,half);
        self.opponent = Player::new(opponent_side,half);
        self.ball = Ball::new(half,height/2);
        self.width = width;
        self.height = height;
        
    }
    pub fn dummy() -> Game{
        Game {
            client : Player::new(Side::Up,0),
            opponent : Player::new(Side::Up,0),
            ball : Ball::new(0,0),
            width : 0,
            height : 0
        }
    }
}


use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};
use rand::Rng;
use std::f64::consts::PI;


struct Block {
    x: usize,
    y: usize,
    width: usize,
    color: Color,
    hit: bool,
}


struct BlockBreaker {
    width: usize,
    height: usize,
    paddle_char: &'static str,
    ball_chars: Vec<&'static str>,
    ball_frame: usize,
    block_char: &'static str,
    paddle_size: usize,

    ball_speed: f64,
    paddle_speed: f64,
    
    paddle_x: f64,
    paddle_y: f64,
    
    ball_x: f64,
    ball_y: f64,
    ball_dx: f64,
    ball_dy: f64,
    
    blocks: Vec<Block>,
    
    score: usize,
    lives: usize,
    game_over: bool,
    game_won: bool,
    last_update: Instant,
    animation_counter: usize,
}



impl BlockBreaker {
    fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        let width = width as usize;
        let height = height as usize;


         
        let paddle_size = 10;
        let paddle_x = (width - paddle_size) as f64 / 2.0;
        let paddle_y = (height - 2) as f64;
        
        let ball_x = width as f64 / 2.0;
        let ball_y = paddle_y - 1.0;
        
        let angle = PI / 4.0;
        let ball_dx = angle.cos();
        let ball_dy = -angle.sin();

        let mut game = BlockBreaker {
            width,
            height,
            paddle_char: "═",
            ball_chars: vec!["O", "@", "●", "*"],
            ball_frame: 0,
            block_char: "█",
            paddle_size,
            
            ball_speed: 20.0,
            paddle_speed: 500.0,
            
            paddle_x,
            paddle_y,
            
            ball_x,
            ball_y,
            ball_dx,
            ball_dy,
            
            blocks: Vec::new(),
            
            score: 0,
            lives: 3,
            game_over: false,
            game_won: false,
            last_update: Instant::now(),
            animation_counter: 0,
        };
        
        game.create_blocks();
        game
    }



    fn create_blocks(&mut self) {
        self.blocks.clear();
        let block_rows = 5;
        let block_cols = self.width / 3;


        for row in 0..block_rows {
            for col in 0..block_cols {
                let color = match row % 3 {
                    0 => Color::Cyan,
                    1 => Color::Blue,
                    2 => Color::Magenta,
                    _ => Color::White,
                };
                let block = Block {
                    x: col * 3,
                    y: row + 3,
                    width: 3,
                    color,
                    hit: false,
                };
                
                self.blocks.push(block);
            }
        }
    }


    fn reset_game(&mut self) {
        self.paddle_x = (self.width - self.paddle_size) as f64 / 2.0;
        self.paddle_y = (self.height - 2) as f64;
        
        self.ball_x = self.width as f64 / 2.0;
        self.ball_y = self.paddle_y - 1.0;

        let angle = PI / 4.0;
        self.ball_dx = angle.cos();
        self.ball_dy = -angle.sin();
        
        self.create_blocks();
        
        self.score = 0;
        self.lives = 3;
        self.game_over = false;
        self.game_won = false;
        self.last_update = Instant::now();
        self.animation_counter = 0;
    }


    fn update_paddle(&mut self, direction: &str, dt: f64) {
        let move_amount = self.paddle_speed * dt;
        if direction == "left" {
            self.paddle_x = (self.paddle_x - move_amount).max(0.0);
        } else if direction == "right" {
            self.paddle_x = (self.paddle_x + move_amount).min((self.width - self.paddle_size) as f64);
        }
    }


    fn update_ball(&mut self, dt: f64) {

        if self.animation_counter % 5 == 0 {
            self.ball_frame = (self.ball_frame + 1) % self.ball_chars.len();
        }
    }
    
}

fn main() {
    println!("Hello, world!");
}

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

    }
}

fn main() {
    println!("Hello, world!");
}

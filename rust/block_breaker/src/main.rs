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
        
        let angle = PI / 4.0; // 45 degrees
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
                    y: row + 3, // Start a few rows down from the top
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
        
        let angle = PI / 4.0; // 45 degrees
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
        

        let new_x = self.ball_x + self.ball_dx * self.ball_speed * dt;
        let new_y = self.ball_y + self.ball_dy * self.ball_speed * dt;
        
        if new_x < 0.0 || new_x >= self.width as f64 {
            self.ball_dx = -self.ball_dx;
            let new_x = new_x.max(0.0).min((self.width - 1) as f64);
            self.ball_x = new_x;
            return;
        }
        
        if new_y < 0.0 {
            self.ball_dy = -self.ball_dy;
            self.ball_y = 0.0;
            return;
        }
        
        if new_y >= self.paddle_y && 
           self.ball_y < self.paddle_y && 
           new_x >= self.paddle_x && 
           new_x < self.paddle_x + self.paddle_size as f64 {

            let hit_position = (new_x - self.paddle_x) / self.paddle_size as f64; // 0.0 to 1.0
            let angle = PI * (0.25 + 0.5 * hit_position); // π/4 to 3π/4
            

            self.ball_dx = angle.cos() * if hit_position >= 0.5 { 1.0 } else { -1.0 };
            self.ball_dy = -angle.sin();
            
            self.ball_y = self.paddle_y - 1.0; // Move ball above paddle
        } else if new_y >= self.height as f64 {

            self.lives -= 1;
            if self.lives <= 0 {
                self.game_over = true;
            } else {

                self.ball_x = self.paddle_x + self.paddle_size as f64 / 2.0;
                self.ball_y = self.paddle_y - 1.0;
                
                // Random angle between π/6 and 5π/6
                let mut rng = rand::thread_rng();
                let angle = PI * (1.0/6.0 + 2.0/3.0 * rng.gen::<f64>());
                self.ball_dx = angle.cos() * if rng.gen::<bool>() { 1.0 } else { -1.0 };
                self.ball_dy = -angle.sin();
            }
            return;
        } else {
            self.ball_x = new_x;
            self.ball_y = new_y;
        }
        
        self.check_block_collisions();
        
        if self.blocks.iter().all(|block| block.hit) {
            self.game_won = true;
        }
    }
    
    fn check_block_collisions(&mut self) {
        let ball_int_x = self.ball_x as usize;
        let ball_int_y = self.ball_y as usize;
        
        for block in &mut self.blocks {
            if block.hit {
                continue;
            }
            
            if ball_int_y == block.y && 
               ball_int_x >= block.x && 
               ball_int_x < block.x + block.width {
                block.hit = true;
                self.score += 10;
                self.ball_dy = -self.ball_dy;
                break;
            }
        }
    }
    
    fn draw(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        

        for i in 0..self.paddle_size {
            let x = (self.paddle_x + i as f64) as u16;
            let y = self.paddle_y as u16;
            if x < self.width as u16 && y < self.height as u16 {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x, y),
                    SetForegroundColor(Color::Green),
                    Print(self.paddle_char),
                )?;
            }
        }
        

        let ball_int_x = self.ball_x as u16;
        let ball_int_y = self.ball_y as u16;
        if ball_int_x < self.width as u16 && ball_int_y < self.height as u16 {
            let current_ball_char = self.ball_chars[self.ball_frame];
            let color = if self.animation_counter % 10 < 5 { Color::White } else { Color::Yellow };
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(ball_int_x, ball_int_y),
                SetForegroundColor(color),
                Print(current_ball_char),
            )?;
        }
        

        for block in &self.blocks {
            if block.hit {
                continue;
            }
            
            for i in 0..block.width {
                let x = (block.x + i) as u16;
                let y = block.y as u16;
                if x < self.width as u16 && y < self.height as u16 {
                    execute!(
                        stdout(),
                        crossterm::cursor::MoveTo(x, y),
                        SetForegroundColor(block.color),
                        Print(self.block_char),
                    )?;
                }
            }
        }
        

        let status_text = format!("Score: {}  Lives: {}", self.score, self.lives);
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Yellow),
            Print(status_text),
        )?;
        

        if self.game_over {
            let game_over_text = "GAME OVER - Press 'r' to restart or 'q' to quit";
            let x = ((self.width as isize - game_over_text.len() as isize) / 2).max(0) as u16;
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x, (self.height / 2) as u16),
                SetForegroundColor(Color::Red),
                Print(game_over_text),
            )?;
        } else if self.game_won {
            let win_text = "YOU WIN! - Press 'r' to restart or 'q' to quit";
            let x = ((self.width as isize - win_text.len() as isize) / 2).max(0) as u16;
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x, (self.height / 2) as u16),
                SetForegroundColor(Color::Yellow),
                Print(win_text),
            )?;
        }
        
        stdout().flush()?;
        Ok(())
    }
    
    fn run(&mut self) -> Result<()> {
        loop {
            let current_time = Instant::now();
            let dt = (current_time - self.last_update).as_secs_f64();
            self.last_update = current_time;
            

            if event::poll(Duration::from_millis(1))? {
                if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => {
                            if self.game_over || self.game_won {
                                self.reset_game();
                            }
                        },
                        KeyCode::Left => {
                            if !self.game_over && !self.game_won {
                                self.update_paddle("left", dt);
                            }
                        },
                        KeyCode::Right => {
                            if !self.game_over && !self.game_won {
                                self.update_paddle("right", dt);
                            }
                        },
                        _ => {}
                    }
                }
            }
            

            if !self.game_over && !self.game_won {
                self.update_ball(dt);
            }
            

            self.draw()?;
            

            self.animation_counter += 1;
            

            std::thread::sleep(Duration::from_millis(10));
        }
        
        Ok(())
    }
}

fn main() -> Result<()> {

    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        EnterAlternateScreen,
        Hide,
    )?;
    

    let mut game = BlockBreaker::new();
    let result = game.run();
    

    execute!(
        stdout(),
        LeaveAlternateScreen,
        Show,
    )?;
    terminal::disable_raw_mode()?;
    
    result
}
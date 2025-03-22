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

fn main() {
    println!("Hello, world!");
}

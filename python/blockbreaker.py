import curses
import random
import time
import math

class BlockBreaker:
    def __init__(self, screen):
        self.screen = screen
        curses.curs_set(0)
        self.screen.timeout(0)
        self.height, self.width = self.screen.getmaxyx()

    self.paddle_char = "═"
        self.ball_char = "●"
        self.block_char = "█"
        self.paddle_size = 8
        
        self.ball_speed = 20
        self.paddle_speed = 30
        
        self.setup_colors()
        
        self.reset_game()
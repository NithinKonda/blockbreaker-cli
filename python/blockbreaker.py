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


    def setup_colors(self):
        if curses.has_colors():
            curses.start_color()
            curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLACK)   # Ball
            curses.init_pair(2, curses.COLOR_GREEN, curses.COLOR_BLACK)   # Paddle
            curses.init_pair(3, curses.COLOR_CYAN, curses.COLOR_BLACK)    # Blocks 1
            curses.init_pair(4, curses.COLOR_BLUE, curses.COLOR_BLACK)    # Blocks 2
            curses.init_pair(5, curses.COLOR_MAGENTA, curses.COLOR_BLACK) # Blocks 3
            curses.init_pair(6, curses.COLOR_RED, curses.COLOR_BLACK)     # Game over
            curses.init_pair(7, curses.COLOR_YELLOW, curses.COLOR_BLACK)  # Score


    def reset_game(self):
        # Reset paddle position
        self.paddle_x = (self.width - self.paddle_size) // 2
        self.paddle_y = self.height - 2
        
        # Reset ball position (starts above paddle)
        self.ball_x = self.width // 2
        self.ball_y = self.paddle_y - 1
        
        # Set ball direction (starts moving up-right)
        angle = math.pi / 4  # 45 degrees
        self.ball_dx = math.cos(angle)
        self.ball_dy = -math.sin(angle)
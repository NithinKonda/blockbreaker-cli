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

        self.paddle_x = (self.width - self.paddle_size) // 2
        self.paddle_y = self.height - 2
        
        self.ball_x = self.width // 2
        self.ball_y = self.paddle_y - 1
        

        angle = math.pi / 4  # 45 degrees
        self.ball_dx = math.cos(angle)
        self.ball_dy = -math.sin(angle)


        self.create_blocks()

        self.score = 0
        self.lives = 3
        self.game_over = False
        self.game_won = False
        self.last_update = time.time()




    def create_blocks(self):
        self.blocks = []
        block_rows = 5
        block_cols = self.width // 3
        

        for row in range(block_rows):
            for col in range(block_cols):
                block = {
                    'x': col * 3,
                    'y': row + 3,
                    'width': 3,
                    'color': (row % 3) + 3,
                    'hit': False
                }
                self.blocks.append(block)

    def update_paddle(self, direction, dt):
        move_amount = int(self.paddle_speed * dt)
        if direction == 'left':
            self.paddle_x = max(0, self.paddle_x - move_amount)
        elif direction == 'right':
            self.paddle_x = min(self.width - self.paddle_size, self.paddle_x + move_amount)


    def update_ball(self, dt):
        new_x = self.ball_x + self.ball_dx * self.ball_speed * dt
        new_y = self.ball_y + self.ball_dy * self.ball_speed * dt
        

        if new_x < 0 or new_x >= self.width:
            self.ball_dx = -self.ball_dx
            new_x = max(0, min(new_x, self.width - 1))
        
        if new_y < 0:
            self.ball_dy = -self.ball_dy
            new_y = 0
        

        if (new_y >= self.paddle_y and 
            self.ball_y < self.paddle_y and 
            new_x >= self.paddle_x and 
            new_x < self.paddle_x + self.paddle_size):
            

            hit_position = (new_x - self.paddle_x) / self.paddle_size
            angle = math.pi * (0.25 + 0.5 * hit_position)  # π/4 to 3π/4
            

            self.ball_dx = math.cos(angle) * (1 if hit_position >= 0.5 else -1)
            self.ball_dy = -math.sin(angle)
            
            new_y = self.paddle_y - 1
         if new_y >= self.height:
            self.lives -= 1
            if self.lives <= 0:
                self.game_over = True
            else:
                # Reset ball position
                self.ball_x = self.paddle_x + self.paddle_size // 2
                self.ball_y = self.paddle_y - 1
                
                # Random angle between π/6 and 5π/6
                angle = math.pi * (1/6 + 2/3 * random.random())
                self.ball_dx = math.cos(angle) * (1 if random.random() > 0.5 else -1)
                self.ball_dy = -math.sin(angle)
                return
        self.ball_x, self.ball_y = new_x, new_y
        

        self.check_block_collisions()
        

        if all(block['hit'] for block in self.blocks):
            self.game_won = True
    


    def check_block_collisions(self):
        for block in self.blocks:
            if block['hit']:
                continue
            

            ball_int_x, ball_int_y = int(self.ball_x), int(self.ball_y)
            
            if (ball_int_y == block['y'] and 
                ball_int_x >= block['x'] and 
                ball_int_x < block['x'] + block['width']):
                

                block['hit'] = True
                

                self.score += 10
                

                self.ball_dy = -self.ball_dy
                

                break


    def draw(self):
        self.screen.clear()
        
        for i in range(self.paddle_size):
            x = int(self.paddle_x + i)
            y = int(self.paddle_y)
            if 0 <= x < self.width and 0 <= y < self.height:
                try:
                    if curses.has_colors():
                        self.screen.addstr(y, x, self.paddle_char, curses.color_pair(2))
                    else:
                        self.screen.addstr(y, x, self.paddle_char)
                except curses.error:
                    pass
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
        self.ball_chars = ["O", "@", "●", "*"]
        self.ball_frame = 0
        self.block_char = "█"
        self.paddle_size = 10
        
        self.ball_speed = 20
        self.paddle_speed = 500
        
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
        self.animation_counter = 0

    def create_blocks(self):
        self.blocks = []
        block_rows = 5
        block_cols = self.width // 3
        

        for row in range(block_rows):
            for col in range(block_cols):
                block = {
                    'x': col * 3,
                    'y': row + 3,  # Start a few rows down from the top
                    'width': 3,
                    'color': (row % 3) + 3,  # Alternate colors (3, 4, 5)
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
        # Update ball animation frame
        if self.animation_counter % 5 == 0:
            self.ball_frame = (self.ball_frame + 1) % len(self.ball_chars)
            
        # Calculate new ball position based on direction and time elapsed
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
            
            # Ball bounces off paddle with angle based on where it hits
            hit_position = (new_x - self.paddle_x) / self.paddle_size  # 0.0 to 1.0
            angle = math.pi * (0.25 + 0.5 * hit_position)  # π/4 to 3π/4
            
            # Update direction (bounce up with new angle)
            self.ball_dx = math.cos(angle) * (1 if hit_position >= 0.5 else -1)
            self.ball_dy = -math.sin(angle)
            
            new_y = self.paddle_y - 1  # Move ball above paddle
        
        # Handle falling below paddle (lose life)
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
        
        # Draw ball
        ball_int_x, ball_int_y = int(self.ball_x), int(self.ball_y)
        if 0 <= ball_int_x < self.width and 0 <= ball_int_y < self.height:
            try:
                # Use the current frame's ball character
                current_ball_char = self.ball_chars[self.ball_frame]
                if curses.has_colors():
                    # Make the ball blink by alternating between two bright colors
                    color = 1 if self.animation_counter % 10 < 5 else 7
                    self.screen.addstr(ball_int_y, ball_int_x, current_ball_char, curses.color_pair(color) | curses.A_BOLD)
                else:
                    self.screen.addstr(ball_int_y, ball_int_x, current_ball_char, curses.A_BOLD)
            except curses.error:
                pass
        
        # Draw blocks
        for block in self.blocks:
            if block['hit']:
                continue
            
            for i in range(block['width']):
                x = block['x'] + i
                y = block['y']
                if 0 <= x < self.width and 0 <= y < self.height:
                    try:
                        if curses.has_colors():
                            self.screen.addstr(y, x, self.block_char, curses.color_pair(block['color']))
                        else:
                            self.screen.addstr(y, x, self.block_char)
                    except curses.error:
                        pass


        status_text = f"Score: {self.score}  Lives: {self.lives}"
        try:
            if curses.has_colors():
                self.screen.addstr(0, 0, status_text, curses.color_pair(7))
            else:
                self.screen.addstr(0, 0, status_text)
        except curses.error:
            pass
        

        if self.game_over:
            game_over_text = "GAME OVER - Press 'r' to restart or 'q' to quit"
            x = max(0, (self.width - len(game_over_text)) // 2)
            try:
                if curses.has_colors():
                    self.screen.addstr(self.height // 2, x, game_over_text, curses.color_pair(6))
                else:
                    self.screen.addstr(self.height // 2, x, game_over_text)
            except curses.error:
                pass
        elif self.game_won:
            win_text = "YOU WIN! - Press 'r' to restart or 'q' to quit"
            x = max(0, (self.width - len(win_text)) // 2)
            try:
                if curses.has_colors():
                    self.screen.addstr(self.height // 2, x, win_text, curses.color_pair(7))
                else:
                    self.screen.addstr(self.height // 2, x, win_text)
            except curses.error:
                pass
        
        self.screen.refresh()



    def run(self):
        while True:
            current_time = time.time()
            dt = current_time - self.last_update
            self.last_update = current_time
            

            key = self.screen.getch()
            if key == ord('q'):
                break
            elif key == ord('r') and (self.game_over or self.game_won):
                self.reset_game()
            

            if not self.game_over and not self.game_won:

                if key == curses.KEY_LEFT:
                    self.update_paddle('left', dt)
                elif key == curses.KEY_RIGHT:
                    self.update_paddle('right', dt)
                

                self.update_ball(dt)
            

            self.draw()
            
            # Increment animation counter
            self.animation_counter += 1
            
            # Cap frame rate
            time.sleep(max(0.01 - (time.time() - current_time), 0))


def main():
    curses.wrapper(lambda screen: BlockBreaker(screen).run())

if __name__ == "__main__":
    main()
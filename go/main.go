package main

import (
	"math"
	"time"

	"github.com/gdamore/tcell/v2"
)

type Block struct {
	x     int
	y     int
	width int
	color tcell.Color
	hit   bool
}

type BlockBreaker struct {
	screen     tcell.Screen
	width      int
	height     int
	paddleChar rune
	ballChars  []rune
	ballFrame  int
	blockChar  rune
	paddleSize int

	ballSpeed   float64
	paddleSpeed float64

	paddleX float64
	paddleY float64

	ballX  float64
	ballY  float64
	ballDX float64
	ballDY float64

	blocks []Block

	score            int
	lives            int
	gameOver         bool
	gameWon          bool
	lastUpdate       time.Time
	animationCounter int
}

func NewBlockBreaker(screen tcell.Screen) *BlockBreaker {
	width, height := screen.Size()

	paddleSize := 10
	paddleX := float64(width-paddleSize) / 2
	paddleY := float64(height - 2)

	ballX := float64(width) / 2
	ballY := paddleY - 1

	angle := math.Pi / 4
	ballDX := math.Cos(angle)
	ballDY := -math.Sin(angle)

	game := &BlockBreaker{
		screen:     screen,
		width:      width,
		height:     height,
		paddleChar: '═',
		ballChars:  []rune{'O', '@', '●', '*'},
		ballFrame:  0,
		blockChar:  '█',
		paddleSize: paddleSize,

		ballSpeed:   20.0,
		paddleSpeed: 500.0,

		paddleX: paddleX,
		paddleY: paddleY,

		ballX:  ballX,
		ballY:  ballY,
		ballDX: ballDX,
		ballDY: ballDY,

		blocks: []Block{},

		score:            0,
		lives:            3,
		gameOver:         false,
		gameWon:          false,
		lastUpdate:       time.Now(),
		animationCounter: 0,
	}

	game.createBlocks()
	return game
}

func (g *BlockBreaker) createBlocks() {
	g.blocks = []Block{}
	blockRows := 5
	blockCols := g.width / 3

	for row := 0; row < blockRows; row++ {
		for col := 0; col < blockCols; col++ {
			var color tcell.Color
			switch row % 3 {
			case 0:
				color = tcell.ColorCyan
			case 1:
				color = tcell.ColorBlue
			case 2:
				color = tcell.ColorPurple
			}

			block := Block{
				x:     col * 3,
				y:     row + 3,
				width: 3,
				color: color,
				hit:   false,
			}

			g.blocks = append(g.blocks, block)
		}
	}
}

func (g *BlockBreaker) resetGame() {
	g.paddleX = float64(g.width-g.paddleSize) / 2
	g.paddleY = float64(g.height - 2)

	g.ballX = float64(g.width) / 2
	g.ballY = g.paddleY - 1

	angle := math.Pi / 4
	g.ballDX = math.Cos(angle)
	g.ballDY = -math.Sin(angle)

	g.createBlocks()

	g.score = 0
	g.lives = 3
	g.gameOver = false
	g.gameWon = false
	g.lastUpdate = time.Now()
	g.animationCounter = 0
}

func (g *BlockBreaker) updatePaddle(direction string, dt float64) {
	moveAmount := g.paddleSpeed * dt
	if direction == "left" {
		g.paddleX = math.Max(0, g.paddleX-moveAmount)
	} else if direction == "right" {
		g.paddleX = math.Min(float64(g.width-g.paddleSize), g.paddleX+moveAmount)
	}
}

func (g *BlockBreaker) updateBall(dt float64) {

	if g.animationCounter%5 == 0 {
		g.ballFrame = (g.ballFrame + 1) % len(g.ballChars)
	}
	newX := g.ballX + g.ballDX*g.ballSpeed*dt
	newY := g.ballY + g.ballDY*g.ballSpeed*dt

	if newX < 0 || newX >= float64(g.width) {
		g.ballDX = -g.ballDX
		newX = math.Max(0, math.Min(newX, float64(g.width-1)))
	}

	if newY < 0 {
		g.ballDY = -g.ballDY
		newY = 0
	}

	if newY >= g.paddleY &&
		g.ballY < g.paddleY &&
		newX >= g.paddleX &&
		newX < g.paddleX+float64(g.paddleSize) {
		hitPosition := (newX - g.paddleX) / float64(g.paddleSize)
		angle := math.Pi * (0.25 + 0.5*hitPosition)

		multiplier := 1.0
		if hitPosition < 0.5 {
			multiplier = -1.0
		}
		g.ballDX = math.Cos(angle) * multiplier
		g.ballDY = -math.Sin(angle)

		newY = g.paddleY - 1 // Move ball above paddle
	}
	else if newY >= float64(g.height) {
		// Handle falling below paddle (lose life)
		g.lives--
		if g.lives <= 0 {
			g.gameOver = true
		} else {
			// Reset ball position
			g.ballX = g.paddleX + float64(g.paddleSize)/2
			g.ballY = g.paddleY - 1
			
			// Random angle between π/6 and 5π/6
			angle := math.Pi * (1.0/6.0 + 2.0/3.0*rand.Float64())
			multiplier := 1.0
			if rand.Float64() > 0.5 {
				multiplier = -1.0
			}
			g.ballDX = math.Cos(angle) * multiplier
			g.ballDY = -math.Sin(angle)
		}
		return
	}
	g.ballX = newX
	g.ballY = newY
	
	g.checkBlockCollisions()
	
	allHit := true
	for _, block := range g.blocks {
		if !block.hit {
			allHit = false
			break
		}
	}
	
	if allHit {
		g.gameWon = true
	}
}


func (g *BlockBreaker) checkBlockCollisions() {
	ballIntX := int(g.ballX)
	ballIntY := int(g.ballY)
	
	for i := range g.blocks {
		if g.blocks[i].hit {
			continue
		}
		
		if ballIntY == g.blocks[i].y && 
		   ballIntX >= g.blocks[i].x && 
		   ballIntX < g.blocks[i].x+g.blocks[i].width {
			g.blocks[i].hit = true
			g.score += 10
			g.ballDY = -g.ballDY
			break
		}
	}
}


func (g *BlockBreaker) draw() {
	g.screen.Clear()
	
	defStyle := tcell.StyleDefault
	
	paddleStyle := defStyle.Foreground(tcell.ColorGreen)
	for i := 0; i < g.paddleSize; i++ {
		x := int(g.paddleX) + i
		y := int(g.paddleY)
		if x >= 0 && x < g.width && y >= 0 && y < g.height {
			g.screen.SetContent(x, y, g.paddleChar, nil, paddleStyle)
		}
	}

	ballIntX := int(g.ballX)
	ballIntY := int(g.ballY)
	if ballIntX >= 0 && ballIntX < g.width && ballIntY >= 0 && ballIntY < g.height {
		currentBallChar := g.ballChars[g.ballFrame]
		ballColor := tcell.ColorWhite
		if g.animationCounter%10 < 5 {
			ballColor = tcell.ColorYellow
		}
		ballStyle := defStyle.Foreground(ballColor)
		g.screen.SetContent(ballIntX, ballIntY, currentBallChar, nil, ballStyle)
	}

	for _, block := range g.blocks {
		if block.hit {
			continue
		}
		
		blockStyle := defStyle.Foreground(block.color)
		for i := 0; i < block.width; i++ {
			x := block.x + i
			y := block.y
			if x >= 0 && x < g.width && y >= 0 && y < g.height {
				g.screen.SetContent(x, y, g.blockChar, nil, blockStyle)
			}
		}
	}

	statusText := fmt.Sprintf("Score: %d  Lives: %d", g.score, g.lives)
	statusStyle := defStyle.Foreground(tcell.ColorYellow)
	for i, r := range statusText {
		if i < g.width {
			g.screen.SetContent(i, 0, r, nil, statusStyle)
		}
	}
}
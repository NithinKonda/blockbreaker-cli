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

}

package main

import (
	"github.com/gdamore/tcell/v2"
)

type Block struct {
	x     int
	y     int
	width int
	color tcell.Color
	hit   bool
}

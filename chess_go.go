// chess_go.go — Шахматы (2D) на Go

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

const SIZE = 8

var board [SIZE][SIZE]rune
var whiteTurn = true

func setupBoard() {
	setup := []string{
		"RNBQKBNR",
		"PPPPPPPP",
		"........",
		"........",
		"........",
		"........",
		"pppppppp",
		"rnbqkbnr",
	}
	for i := 0; i < SIZE; i++ {
		for j := 0; j < SIZE; j++ {
			board[i][j] = rune(setup[i][j])
		}
	}
}

func printBoard() {
	fmt.Println("  a b c d e f g h")
	for i := 0; i < SIZE; i++ {
		fmt.Printf("%d ", 8-i)
		for j := 0; j < SIZE; j++ {
			fmt.Printf("%c ", board[i][j])
		}
		fmt.Println()
	}
	fmt.Println("  a b c d e f g h")
}

func movePiece(from, to string) bool {
	fx := int(from[0] - 'a')
	fy := int('8' - from[1])
	tx := int(to[0] - 'a')
	ty := int('8' - to[1])
	if fx < 0 || fx >= SIZE || fy < 0 || fy >= SIZE || tx < 0 || tx >= SIZE || ty < 0 || ty >= SIZE {
		return false
	}
	piece := board[fy][fx]
	if piece == '.' {
		return false
	}
	target := board[ty][tx]
	if target != '.' && (unicode.IsUpper(target) == unicode.IsUpper(piece)) {
		return false
	}
	board[ty][tx] = piece
	board[fy][fx] = '.'
	return true
}

func main() {
	setupBoard()
	reader := bufio.NewReader(os.Stdin)
	for {
		printBoard()
		turn := "Белые"
		if !whiteTurn {
			turn = "Чёрные"
		}
		fmt.Printf("%s ходят. Введите ход (e2 e4): ", turn)
		input, _ := reader.ReadString('\n')
		input = strings.TrimSpace(input)
		if input == "exit" {
			break
		}
		parts := strings.Fields(input)
		if len(parts) != 2 {
			fmt.Println("Неверный формат")
			continue
		}
		if movePiece(parts[0], parts[1]) {
			whiteTurn = !whiteTurn
		} else {
			fmt.Println("Неверный ход.")
		}
	}
}

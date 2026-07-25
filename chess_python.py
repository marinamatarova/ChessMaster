# chess_python.py — Шахматы (2D) на Python

import sys

class Board:
    def __init__(self):
        self.board = [['.' for _ in range(8)] for _ in range(8)]
        self.turn = 'white'
        self.setup()

    def setup(self):
        setup = [
            "RNBQKBNR",
            "PPPPPPPP",
            "........",
            "........",
            "........",
            "........",
            "pppppppp",
            "rnbqkbnr"
        ]
        for i in range(8):
            for j in range(8):
                self.board[i][j] = setup[i][j]
        self.turn = 'white'

    def print_board(self):
        print('  a b c d e f g h')
        for i, row in enumerate(self.board):
            print(8 - i, ' '.join(row))
        print('  a b c d e f g h')

    def get_piece(self, col, row):
        x = ord(col) - ord('a')
        y = 8 - row
        return self.board[y][x]

    def is_white(self, piece):
        return piece.isupper()

    def is_black(self, piece):
        return piece.islower()

    def in_bounds(self, x, y):
        return 0 <= x < 8 and 0 <= y < 8

    def move_piece(self, from_sq, to_sq):
        fx = ord(from_sq[0]) - ord('a')
        fy = 8 - int(from_sq[1])
        tx = ord(to_sq[0]) - ord('a')
        ty = 8 - int(to_sq[1])
        if not self.in_bounds(fx, fy) or not self.in_bounds(tx, ty):
            return False
        piece = self.board[fy][fx]
        if piece == '.':
            return False
        if self.turn == 'white' and not self.is_white(piece):
            return False
        if self.turn == 'black' and not self.is_black(piece):
            return False
        target = self.board[ty][tx]
        if target != '.' and (self.is_white(target) == self.is_white(piece)):
            return False
        # Временно перемещаем
        self.board[ty][tx] = piece
        self.board[fy][fx] = '.'
        self.turn = 'black' if self.turn == 'white' else 'white'
        return True

    def is_in_check(self, color):
        # Упрощённо: не реализовано
        return False

def main():
    board = Board()
    while True:
        board.print_board()
        print(f"Ход {board.turn}.")
        move = input("Введите ход (например, e2 e4): ").strip()
        if move.lower() == 'exit':
            break
        parts = move.split()
        if len(parts) != 2:
            print("Неверный формат. Введите два поля, например: e2 e4")
            continue
        from_sq, to_sq = parts[0].lower(), parts[1].lower()
        if board.move_piece(from_sq, to_sq):
            print("Ход выполнен.")
        else:
            print("Неверный ход.")

if __name__ == "__main__":
    main()

// ChessCSharp.cs — Шахматы (2D) на C#

using System;

class ChessCSharp {
    static char[,] board = new char[8, 8];
    static bool whiteTurn = true;

    static void SetupBoard() {
        string[] setup = {
            "RNBQKBNR",
            "PPPPPPPP",
            "........",
            "........",
            "........",
            "........",
            "pppppppp",
            "rnbqkbnr"
        };
        for (int i = 0; i < 8; i++)
            for (int j = 0; j < 8; j++)
                board[i, j] = setup[i][j];
    }

    static void PrintBoard() {
        Console.WriteLine("  a b c d e f g h");
        for (int i = 0; i < 8; i++) {
            Console.Write((8 - i) + " ");
            for (int j = 0; j < 8; j++)
                Console.Write(board[i, j] + " ");
            Console.WriteLine();
        }
        Console.WriteLine("  a b c d e f g h");
    }

    static bool MovePiece(string from, string to) {
        int fx = from[0] - 'a';
        int fy = '8' - from[1];
        int tx = to[0] - 'a';
        int ty = '8' - to[1];
        if (fx < 0 || fx >= 8 || fy < 0 || fy >= 8 || tx < 0 || tx >= 8 || ty < 0 || ty >= 8)
            return false;
        char piece = board[fy, fx];
        if (piece == '.') return false;
        char target = board[ty, tx];
        if (target != '.' && (char.IsUpper(target) == char.IsUpper(piece))) return false;
        board[ty, tx] = piece;
        board[fy, fx] = '.';
        return true;
    }

    static void Main() {
        SetupBoard();
        while (true) {
            PrintBoard();
            Console.Write((whiteTurn ? "Белые" : "Чёрные") + " ходят. Введите ход (e2 e4): ");
            string input = Console.ReadLine();
            if (input == "exit") break;
            string[] parts = input.Split(' ');
            if (parts.Length != 2) { Console.WriteLine("Неверный формат"); continue; }
            if (MovePiece(parts[0], parts[1])) {
                whiteTurn = !whiteTurn;
            } else {
                Console.WriteLine("Неверный ход.");
            }
        }
    }
}

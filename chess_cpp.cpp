// chess_cpp.cpp — Шахматы (2D) на C++

#include <iostream>
#include <string>
#include <vector>
#include <cctype>
using namespace std;

const int SIZE = 8;
vector<vector<char>> board(SIZE, vector<char>(SIZE, '.'));

void setup_board() {
    vector<string> setup = {
        "RNBQKBNR",
        "PPPPPPPP",
        "........",
        "........",
        "........",
        "........",
        "pppppppp",
        "rnbqkbnr"
    };
    for (int i = 0; i < SIZE; ++i)
        for (int j = 0; j < SIZE; ++j)
            board[i][j] = setup[i][j];
}

void print_board() {
    cout << "  a b c d e f g h\n";
    for (int i = 0; i < SIZE; ++i) {
        cout << 8 - i << " ";
        for (int j = 0; j < SIZE; ++j) {
            cout << board[i][j] << " ";
        }
        cout << "\n";
    }
    cout << "  a b c d e f g h\n";
}

bool is_white(char p) { return isupper(p); }
bool is_black(char p) { return islower(p); }

int col_index(char c) { return c - 'a'; }
int row_index(char r) { return '8' - r; }

bool move_piece(string from, string to) {
    int fx = col_index(from[0]);
    int fy = row_index(from[1]);
    int tx = col_index(to[0]);
    int ty = row_index(to[1]);
    if (fx < 0 || fx >= SIZE || fy < 0 || fy >= SIZE || tx < 0 || tx >= SIZE || ty < 0 || ty >= SIZE)
        return false;
    char piece = board[fy][fx];
    if (piece == '.') return false;
    char target = board[ty][tx];
    if (target != '.' && (is_white(target) == is_white(piece))) return false;
    board[ty][tx] = piece;
    board[fy][fx] = '.';
    return true;
}

int main() {
    setup_board();
    string from, to;
    bool white_turn = true;
    while (true) {
        print_board();
        cout << (white_turn ? "Белые" : "Чёрные") << " ходят. Введите ход (e2 e4): ";
        cin >> from >> to;
        if (from == "exit") break;
        if (move_piece(from, to)) {
            white_turn = !white_turn;
        } else {
            cout << "Неверный ход.\n";
        }
    }
    return 0;
}

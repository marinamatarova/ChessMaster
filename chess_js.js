// chess_js.js — Шахматы (2D) на JavaScript (Node.js)

const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

const SIZE = 8;
let board = [];
let whiteTurn = true;

function setupBoard() {
    const setup = [
        "RNBQKBNR",
        "PPPPPPPP",
        "........",
        "........",
        "........",
        "........",
        "pppppppp",
        "rnbqkbnr"
    ];
    board = setup.map(row => row.split(''));
}

function printBoard() {
    console.log('  a b c d e f g h');
    for (let i = 0; i < SIZE; i++) {
        process.stdout.write((8 - i) + ' ');
        for (let j = 0; j < SIZE; j++) {
            process.stdout.write(board[i][j] + ' ');
        }
        console.log();
    }
    console.log('  a b c d e f g h');
}

function movePiece(from, to) {
    const fx = from.charCodeAt(0) - 97;
    const fy = 8 - parseInt(from[1]);
    const tx = to.charCodeAt(0) - 97;
    const ty = 8 - parseInt(to[1]);
    if (fx < 0 || fx >= SIZE || fy < 0 || fy >= SIZE || tx < 0 || tx >= SIZE || ty < 0 || ty >= SIZE)
        return false;
    const piece = board[fy][fx];
    if (piece === '.') return false;
    const target = board[ty][tx];
    if (target !== '.' && (target === target.toUpperCase()) === (piece === piece.toUpperCase())) {
        return false;
    }
    board[ty][tx] = piece;
    board[fy][fx] = '.';
    return true;
}

function ask() {
    printBoard();
    const turn = whiteTurn ? 'Белые' : 'Чёрные';
    rl.question(`${turn} ходят. Введите ход (e2 e4): `, (input) => {
        if (input.trim() === 'exit') {
            rl.close();
            return;
        }
        const parts = input.trim().split(/\s+/);
        if (parts.length !== 2) {
            console.log('Неверный формат');
            ask();
            return;
        }
        if (movePiece(parts[0], parts[1])) {
            whiteTurn = !whiteTurn;
        } else {
            console.log('Неверный ход.');
        }
        ask();
    });
}

setupBoard();
ask();

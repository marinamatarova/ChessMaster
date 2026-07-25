// ChessJava.java — Шахматы (2D) на Java (Swing)

import javax.swing.*;
import java.awt.*;

public class ChessJava extends JFrame {
    private static final int SIZE = 8;
    private char[][] board = new char[SIZE][SIZE];

    public ChessJava() {
        setTitle("♟️ ChessMaster");
        setSize(600, 600);
        setDefaultCloseOperation(EXIT_ON_CLOSE);
        setupBoard();
        add(new BoardPanel());
        setVisible(true);
    }

    private void setupBoard() {
        String[] setup = {
            "RNBQKBNR",
            "PPPPPPPP",
            "........",
            "........",
            "........",
            "........",
            "pppppppp",
            "rnbqkbnr"
        };
        for (int i = 0; i < SIZE; i++)
            for (int j = 0; j < SIZE; j++)
                board[i][j] = setup[i].charAt(j);
    }

    class BoardPanel extends JPanel {
        @Override
        protected void paintComponent(Graphics g) {
            super.paintComponent(g);
            int cellSize = getWidth() / SIZE;
            for (int i = 0; i < SIZE; i++)
                for (int j = 0; j < SIZE; j++) {
                    g.setColor((i + j) % 2 == 0 ? Color.WHITE : Color.GRAY);
                    g.fillRect(j * cellSize, i * cellSize, cellSize, cellSize);
                    String piece = String.valueOf(board[i][j]);
                    if (!piece.equals(".")) {
                        g.setColor(Character.isUpperCase(board[i][j]) ? Color.BLACK : Color.RED);
                        g.setFont(new Font("Serif", Font.BOLD, cellSize - 10));
                        g.drawString(piece, j * cellSize + 10, i * cellSize + cellSize - 10);
                    }
                }
        }
    }

    public static void main(String[] args) {
        new ChessJava();
    }
}

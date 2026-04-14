class Solution {
    /**
     * @param {character[][]} board
     * @return {boolean}
     */
    isValidSudoku(board) {
      const rows = new Map();
      const cols = new Map();
      const squares = new Map();

      for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
          const value = board[r][c];

          if (value === ".") {
            continue;
          }

          const squareKey = `${Math.floor(r / 3)},${Math.floor(c / 3)}`;

          if (!rows.has(r)) rows.set(r, new Set());
          if (!cols.has(c)) cols.set(c, new Set());
          if (!squares.has(squareKey)) squares.set(squareKey, new Set());

          if (
            rows.get(r).has(value) ||
            cols.get(c).has(value) ||
            squares.get(squareKey).has(value)
          ) {
            return false;
          }

          rows.get(r).add(value);
          cols.get(c).add(value);
          squares.get(squareKey).add(value);
        }
      }

      return true;
    }
  }

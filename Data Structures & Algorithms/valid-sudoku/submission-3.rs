impl Solution {
      pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
          let mut rows: Vec<HashSet<char>> = vec![HashSet::new();
  9];
          let mut cols: Vec<HashSet<char>> = vec![HashSet::new();
  9];
          let mut squares: Vec<HashSet<char>> = vec!
  [HashSet::new(); 9];

          for r in 0..9 {
              for c in 0..9 {
                  let value = board[r][c];

                  if value == '.' {
                      continue;
                  }

                  let square_index = (r / 3) * 3 + (c / 3);

                  if rows[r].contains(&value)
                      || cols[c].contains(&value)
                      || squares[square_index].contains(&value)
                  {
                      return false;
                  }

                  rows[r].insert(value);
                  cols[c].insert(value);
                  squares[square_index].insert(value);
              }
          }

          true
      }
  }
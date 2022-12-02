struct Solution {}

impl Solution {
    pub fn has_duplicate(v: &Vec<char>) -> bool {
        for (i, c) in v.iter().enumerate() {
            if *c == '.' {
                continue;
            }

            for (j, c2) in v.iter().enumerate() {
                if i == j {
                    continue;
                }
                if *c == *c2 {
                    println!("{} is equal to {}", *c, *c2);
                    return true;
                }
            }
        }
        false
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // check the rows 
        let mut columns = [['.'; 9]; 9];
        let mut squares = [['.'; 9]; 9];
        for (i, row) in board.iter().enumerate() {
            // fill the columns and squares 
            for (j, c) in row.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                columns[j][i] = *c;
                squares[(i/3)*3+ j/3][j%3 + ((i%3)*3)] = *c;
            }
            if Solution::has_duplicate(&row) {
                return false;
            }
        }

        println!("Columns: {:?}", columns);
        println!("Squares: {:?}", squares);

        for i in 0..9 {
            if Solution::has_duplicate(&columns[i].to_vec()) || Solution::has_duplicate(&squares[i].to_vec()) {
                return false;
            }
        }

        true
    }
}

fn main() {
    let board = 
    vec![vec!['5','3','.','.','7','.','.','.','.']
        ,vec!['6','.','.','1','9','5','.','.','.']
        ,vec!['.','9','8','.','.','.','.','6','.']
        ,vec!['8','.','.','.','6','.','.','.','3']
        ,vec!['4','.','.','8','.','3','.','.','1']
        ,vec!['7','.','.','.','2','.','.','.','6']
        ,vec!['.','6','.','.','.','.','2','8','.']
        ,vec!['.','.','.','4','1','9','.','.','5']
        ,vec!['.','.','.','.','8','.','.','7','9']];
    
    println!("{}", Solution::is_valid_sudoku(board));

}

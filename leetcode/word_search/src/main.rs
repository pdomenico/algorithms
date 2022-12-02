struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let first_letter = word.chars().nth(0).unwrap();

        for (i, row) in board.iter().enumerate() {
            for (j, letter) in row.iter().enumerate() {
                if *letter == first_letter {
                    println!("Found first letter at coordinates i={i} j={j}");
                    let mut new_board = board.clone();
                    new_board[i][j] = ' ';
                    if Solution::find_rest_of_word(new_board, &word.as_str()[1..], (i, j)) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn find_rest_of_word(board: Vec<Vec<char>>, word: &str, coord: (usize, usize)) -> bool {
        println!("Now searching for subword {word}");
        if word == "" {
            return true;
        }
        
        let letter = word.chars().nth(0).unwrap();
        // search vertically
        if coord.1 > 0 && board[coord.0][coord.1 -1] == letter {
            println!("Found letter {letter}");
            let mut new_board = board.clone();
            new_board[coord.0][coord.1-1] = ' ';
            if Solution::find_rest_of_word(new_board, &word[1..], (coord.0, coord.1 -1)) {
                return true;
            }
        }

        if coord.1 < board[coord.0].len()-1 && board[coord.0][coord.1 +1] == letter {
            println!("Found letter {letter}");
            let mut new_board = board.clone();
            new_board[coord.0][coord.1+1] = ' ';
            if Solution::find_rest_of_word(new_board, &word[1..], (coord.0, coord.1 +1)) {
                return true;
            }
        }

        // search horizontally
        if coord.0 > 0 && board[coord.0 -1][coord.1] == letter {
            println!("Found letter {letter}");
            let mut new_board = board.clone();
            new_board[coord.0-1][coord.1] = ' ';
            if Solution::find_rest_of_word(new_board, &word[1..], (coord.0 -1, coord.1)) {
                return true;
            }
        }

        if coord.0 < board.len() -1 && board[coord.0 +1][coord.1] == letter {
            println!("Found letter {letter}");
            let mut new_board = board.clone();
            new_board[coord.0+1][coord.1] = ' ';
            if Solution::find_rest_of_word(new_board, &word[1..], (coord.0 +1, coord.1)) {
                return true;
            }
        }

           false
    }

}

fn main() {
    let board = vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'E', 'S'], vec!['A', 'D', 'E', 'E']];
    // let board = vec![vec!['a', 'a']];
    let word = String::from("ABCESEEEFS");

    println!("{}", Solution::exist(board, word));
}
